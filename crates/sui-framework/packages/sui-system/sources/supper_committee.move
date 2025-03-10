module sui_system::supper_committee{

    use std::type_name;
    use std::ascii::String;
    use sui::event;
    use sui::bag::{Self,Bag};
    use sui::vec_set::{Self,VecSet};
    use sui::clock::Clock;
    use sui::address;

    use sui::dynamic_field as df;

    const Timeout:u64 = 7 * 24 * 60 * 60 * 1000;


    public struct ActionKey has store,copy,drop {}


    const Denominator:u64 = 100;
    const Base_Quorum_Proportion:u64 = 50; 
    const Supper_Committee_Quorum_Proportion:u64 = 66;


    /// proposal status 
    const PROPOSAl_STATUS_PENDING: u8 = 1;
    const PROPOSAl_STATUS_ACTIVE:u8 = 2;
    const PROPOSAl_STATUS_PASS:u8 = 3;
    const PROPOSAl_STATUS_FAIL:u8 = 4;
    const PROPOSAl_STATUS_TIMEOUT:u8 = 5;


    public struct UpdateCommitteeValidatorAction has store,copy,drop{
        operate: bool,
        committee_validator: address
    }


    public struct SupperCommittee has store {
        committee_validators: VecSet<address>,
        proposal_list: vector<ID>,
        /// Any extra fields that's not defined statically.
        extra_fields: Bag,
    }

    public struct Proposal has key{
        id: UID,
        /// creator of the proposal
        proposer: address,
        /// count of voters who agree with the proposal
        for_votes: VecSet<address>,
        /// count of voters who're against the proposal
        against_votes: VecSet<address>,
        start_time_ms: u64,
        end_time_ms: u64,
        action_type:String,
        status: u8,
    }

    // Errors
    const ENotExistsCommitteeAddress:u64 = 1;
    const ENotProposalStatusProgress:u64 = 2;
    const ECommitteeValidatorAlreadyExists:u64 = 3;
    const ECommitteeValidatorNotExists:u64 = 4;
    const ENotSupportStructType:u64 = 5;


    public struct CreateProposalEvent has copy,drop {
        proposal_id: ID,
        proposer: address, 
        action_type: String
    }

    public struct VoteProposalEvent has copy,drop {
        proposal_id: ID,
        voter: address,
        agree: bool,
        status: u8,
    }



    public(package) fun new(
        init_committe_validator: vector<address>,
        ctx: &mut TxContext,
    ):SupperCommittee{
        let mut committee_validators = vec_set::empty<address>();

        init_committe_validator.do!(|val| {
            committee_validators.insert(val);
        });

        SupperCommittee{
            committee_validators ,
            proposal_list:vector::empty(),
            extra_fields :bag::new(ctx)
        }
    }

    public(package) fun create_update_committee_validator_proposal(
        self: &mut SupperCommittee,
        operate: bool,
        committee_validator: address ,
        clock: &Clock,
        ctx: &mut TxContext
    ){
        if(operate){
            assert!(!self.committee_validators.contains(&committee_validator),ECommitteeValidatorAlreadyExists);
        }else {
            assert!(self.committee_validators.contains(&committee_validator),ECommitteeValidatorNotExists);
        };
        self.create_proposal(UpdateCommitteeValidatorAction{operate,committee_validator}, clock, ctx);
    }

    public(package) fun execute_update_committee_validator_action(self:&mut SupperCommittee,action: &UpdateCommitteeValidatorAction){
        if(action.operate){
            assert!(!self.committee_validators.contains(&action.committee_validator),ECommitteeValidatorAlreadyExists);
            self.committee_validators.insert(action.committee_validator)
        }else {
            assert!(self.committee_validators.contains(&action.committee_validator),ECommitteeValidatorNotExists);
            self.committee_validators.remove(&action.committee_validator)
        }
    }


    public(package) fun vote_proposal(
        self: &SupperCommittee,
        proposal: &mut Proposal,
        agree: bool,
        clock: &Clock,
        ctx: &TxContext,
    ){
        let sender = ctx.sender();
        assert!(self.committee_validators.contains(&sender),ENotExistsCommitteeAddress);

        assert!(proposal.proposal_status(clock) == PROPOSAl_STATUS_ACTIVE,ENotProposalStatusProgress);

        let proportion =  if (proposal.action_type == type_name::get<UpdateCommitteeValidatorAction>().into_string()){
            self.committee_validators.size() * Supper_Committee_Quorum_Proportion / Denominator
        }else {
            self.committee_validators.size() * Base_Quorum_Proportion / Denominator
        };

        if(agree){
            proposal.for_votes.insert(sender);
        }else {
            proposal.against_votes.insert(sender);
        };
       
        if(proposal.for_votes.size() >= proportion){
            proposal.status = PROPOSAl_STATUS_PASS;
        }else if (proposal.against_votes.size() > proportion){
            proposal.status = PROPOSAl_STATUS_FAIL; 
        };


        let vote_event = VoteProposalEvent{
            proposal_id: object::id(proposal),
            voter: sender,
            agree,
            status: proposal.status
        };

        event::emit(vote_event);
    }

    public fun is_committee_validator(self: &SupperCommittee,addr: address):bool{
        self.committee_validators.contains(&addr)
    }

    public fun proposal_status(self: &Proposal,clock: &Clock):u8{
        if(self.start_time_ms > clock.timestamp_ms()){
            PROPOSAl_STATUS_PENDING
        }else if(self.status ==  PROPOSAl_STATUS_ACTIVE && clock.timestamp_ms() > self.end_time_ms){
            PROPOSAl_STATUS_TIMEOUT
        }else {
            self.status
        }
    }

    public fun proposal_action_type(self: &Proposal):String{
        self.action_type
    }


    public fun proposal_status_pass():u8{
        PROPOSAl_STATUS_PASS
    }


    public fun action<Action:store>(self: &Proposal):&Action{
        df::borrow<ActionKey,Action>(&self.id, ActionKey{})
    }


    public(package) fun create_proposal<Action:store>(
        self: &mut SupperCommittee,
        action: Action,
        clock: &Clock,
        ctx: &mut TxContext,
    ){
        let sender = ctx.sender();
        assert!(self.committee_validators.contains(&sender),ENotExistsCommitteeAddress);
        let action_type = type_name::get<Action>();
        // only sui_system action struct types
        assert!(action_type.get_address() == address::to_ascii_string(@0x3),ENotSupportStructType);

        let mut proposal = Proposal{
            id: object::new(ctx),
            proposer: sender,
            for_votes:vec_set::empty(),
            against_votes:vec_set::empty(),
            start_time_ms: clock.timestamp_ms(),
            end_time_ms:clock.timestamp_ms() + Timeout,
            status: PROPOSAl_STATUS_ACTIVE,
            action_type: action_type.into_string(),
        };

        proposal.for_votes.insert(sender);

        let create_proposal_event = CreateProposalEvent{
            proposal_id: object::id(&proposal),
            proposer: proposal.proposer,
            action_type: proposal.action_type
        };

        
        df::add(&mut proposal.id, ActionKey{}, action);

        self.proposal_list.push_back(object::id(&proposal));

        transfer::share_object(proposal);

        event::emit(create_proposal_event);
    }


}