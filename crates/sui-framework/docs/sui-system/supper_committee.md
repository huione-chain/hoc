---
title: Module `0x3::supper_committee`
---



-  [Struct `ActionKey`](#0x3_supper_committee_ActionKey)
-  [Struct `UpdateCommitteeValidatorAction`](#0x3_supper_committee_UpdateCommitteeValidatorAction)
-  [Struct `SupperCommittee`](#0x3_supper_committee_SupperCommittee)
-  [Resource `Proposal`](#0x3_supper_committee_Proposal)
-  [Struct `CreateProposalEvent`](#0x3_supper_committee_CreateProposalEvent)
-  [Struct `VoteProposalEvent`](#0x3_supper_committee_VoteProposalEvent)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x3_supper_committee_new)
-  [Function `create_update_committee_validator_proposal`](#0x3_supper_committee_create_update_committee_validator_proposal)
-  [Function `execute_update_committee_validator_action`](#0x3_supper_committee_execute_update_committee_validator_action)
-  [Function `vote_proposal`](#0x3_supper_committee_vote_proposal)
-  [Function `is_committee_validator`](#0x3_supper_committee_is_committee_validator)
-  [Function `proposal_status`](#0x3_supper_committee_proposal_status)
-  [Function `proposal_action_type`](#0x3_supper_committee_proposal_action_type)
-  [Function `proposal_status_pass`](#0x3_supper_committee_proposal_status_pass)
-  [Function `action`](#0x3_supper_committee_action)
-  [Function `create_proposal`](#0x3_supper_committee_create_proposal)


<pre><code><b>use</b> <a href="../move-stdlib/ascii.md#0x1_ascii">0x1::ascii</a>;
<b>use</b> <a href="../move-stdlib/type_name.md#0x1_type_name">0x1::type_name</a>;
<b>use</b> <a href="../move-stdlib/vector.md#0x1_vector">0x1::vector</a>;
<b>use</b> <a href="../sui-framework/address.md#0x2_address">0x2::address</a>;
<b>use</b> <a href="../sui-framework/bag.md#0x2_bag">0x2::bag</a>;
<b>use</b> <a href="../sui-framework/clock.md#0x2_clock">0x2::clock</a>;
<b>use</b> <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field">0x2::dynamic_field</a>;
<b>use</b> <a href="../sui-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../sui-framework/vec_set.md#0x2_vec_set">0x2::vec_set</a>;
</code></pre>



<a name="0x3_supper_committee_ActionKey"></a>

## Struct `ActionKey`



<pre><code><b>struct</b> <a href="supper_committee.md#0x3_supper_committee_ActionKey">ActionKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dummy_field: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_supper_committee_UpdateCommitteeValidatorAction"></a>

## Struct `UpdateCommitteeValidatorAction`



<pre><code><b>struct</b> <a href="supper_committee.md#0x3_supper_committee_UpdateCommitteeValidatorAction">UpdateCommitteeValidatorAction</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>operate: bool</code>
</dt>
<dd>

</dd>
<dt>
<code>committee_validator: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_supper_committee_SupperCommittee"></a>

## Struct `SupperCommittee`



<pre><code><b>struct</b> <a href="supper_committee.md#0x3_supper_committee_SupperCommittee">SupperCommittee</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>committee_validators: <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>proposal_list: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>extra_fields: <a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="0x3_supper_committee_Proposal"></a>

## Resource `Proposal`



<pre><code><b>struct</b> <a href="supper_committee.md#0x3_supper_committee_Proposal">Proposal</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>proposer: <b>address</b></code>
</dt>
<dd>
 creator of the proposal
</dd>
<dt>
<code>for_votes: <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 count of voters who agree with the proposal
</dd>
<dt>
<code>against_votes: <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 count of voters who're against the proposal
</dd>
<dt>
<code>start_time_ms: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>end_time_ms: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>action_type: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>status: u8</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_supper_committee_CreateProposalEvent"></a>

## Struct `CreateProposalEvent`



<pre><code><b>struct</b> <a href="supper_committee.md#0x3_supper_committee_CreateProposalEvent">CreateProposalEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>proposer: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>action_type: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_supper_committee_VoteProposalEvent"></a>

## Struct `VoteProposalEvent`



<pre><code><b>struct</b> <a href="supper_committee.md#0x3_supper_committee_VoteProposalEvent">VoteProposalEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>voter: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>agree: bool</code>
</dt>
<dd>

</dd>
<dt>
<code>status: u8</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x3_supper_committee_Base_Quorum_Proportion"></a>



<pre><code><b>const</b> <a href="supper_committee.md#0x3_supper_committee_Base_Quorum_Proportion">Base_Quorum_Proportion</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 50;
</code></pre>



<a name="0x3_supper_committee_Denominator"></a>



<pre><code><b>const</b> <a href="supper_committee.md#0x3_supper_committee_Denominator">Denominator</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 100;
</code></pre>



<a name="0x3_supper_committee_ECommitteeValidatorAlreadyExists"></a>



<pre><code><b>const</b> <a href="supper_committee.md#0x3_supper_committee_ECommitteeValidatorAlreadyExists">ECommitteeValidatorAlreadyExists</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 3;
</code></pre>



<a name="0x3_supper_committee_ECommitteeValidatorNotExists"></a>



<pre><code><b>const</b> <a href="supper_committee.md#0x3_supper_committee_ECommitteeValidatorNotExists">ECommitteeValidatorNotExists</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 4;
</code></pre>



<a name="0x3_supper_committee_ENotExistsCommitteeAddress"></a>



<pre><code><b>const</b> <a href="supper_committee.md#0x3_supper_committee_ENotExistsCommitteeAddress">ENotExistsCommitteeAddress</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0x3_supper_committee_ENotProposalStatusProgress"></a>



<pre><code><b>const</b> <a href="supper_committee.md#0x3_supper_committee_ENotProposalStatusProgress">ENotProposalStatusProgress</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 2;
</code></pre>



<a name="0x3_supper_committee_ENotSupportStructType"></a>



<pre><code><b>const</b> <a href="supper_committee.md#0x3_supper_committee_ENotSupportStructType">ENotSupportStructType</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 5;
</code></pre>



<a name="0x3_supper_committee_PROPOSAl_STATUS_ACTIVE"></a>



<pre><code><b>const</b> <a href="supper_committee.md#0x3_supper_committee_PROPOSAl_STATUS_ACTIVE">PROPOSAl_STATUS_ACTIVE</a>: u8 = 2;
</code></pre>



<a name="0x3_supper_committee_PROPOSAl_STATUS_FAIL"></a>



<pre><code><b>const</b> <a href="supper_committee.md#0x3_supper_committee_PROPOSAl_STATUS_FAIL">PROPOSAl_STATUS_FAIL</a>: u8 = 4;
</code></pre>



<a name="0x3_supper_committee_PROPOSAl_STATUS_PASS"></a>



<pre><code><b>const</b> <a href="supper_committee.md#0x3_supper_committee_PROPOSAl_STATUS_PASS">PROPOSAl_STATUS_PASS</a>: u8 = 3;
</code></pre>



<a name="0x3_supper_committee_PROPOSAl_STATUS_PENDING"></a>

proposal status


<pre><code><b>const</b> <a href="supper_committee.md#0x3_supper_committee_PROPOSAl_STATUS_PENDING">PROPOSAl_STATUS_PENDING</a>: u8 = 1;
</code></pre>



<a name="0x3_supper_committee_PROPOSAl_STATUS_TIMEOUT"></a>



<pre><code><b>const</b> <a href="supper_committee.md#0x3_supper_committee_PROPOSAl_STATUS_TIMEOUT">PROPOSAl_STATUS_TIMEOUT</a>: u8 = 5;
</code></pre>



<a name="0x3_supper_committee_Supper_Committee_Quorum_Proportion"></a>



<pre><code><b>const</b> <a href="supper_committee.md#0x3_supper_committee_Supper_Committee_Quorum_Proportion">Supper_Committee_Quorum_Proportion</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 66;
</code></pre>



<a name="0x3_supper_committee_Timeout"></a>



<pre><code><b>const</b> <a href="supper_committee.md#0x3_supper_committee_Timeout">Timeout</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 604800000;
</code></pre>



<a name="0x3_supper_committee_new"></a>

## Function `new`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="supper_committee.md#0x3_supper_committee_new">new</a>(init_committe_validator: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="supper_committee.md#0x3_supper_committee_SupperCommittee">supper_committee::SupperCommittee</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="supper_committee.md#0x3_supper_committee_new">new</a>(
    init_committe_validator: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;,
    ctx: &<b>mut</b> TxContext,
):<a href="supper_committee.md#0x3_supper_committee_SupperCommittee">SupperCommittee</a>{
    <b>let</b> <b>mut</b> committee_validators = <a href="../sui-framework/vec_set.md#0x2_vec_set_empty">vec_set::empty</a>&lt;<b>address</b>&gt;();

    init_committe_validator.do!(|val| {
        committee_validators.insert(val);
    });

    <a href="supper_committee.md#0x3_supper_committee_SupperCommittee">SupperCommittee</a>{
        committee_validators ,
        proposal_list:<a href="../move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>(),
        extra_fields :<a href="../sui-framework/bag.md#0x2_bag_new">bag::new</a>(ctx)
    }
}
</code></pre>



</details>

<a name="0x3_supper_committee_create_update_committee_validator_proposal"></a>

## Function `create_update_committee_validator_proposal`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="supper_committee.md#0x3_supper_committee_create_update_committee_validator_proposal">create_update_committee_validator_proposal</a>(self: &<b>mut</b> <a href="supper_committee.md#0x3_supper_committee_SupperCommittee">supper_committee::SupperCommittee</a>, operate: bool, committee_validator: <b>address</b>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="supper_committee.md#0x3_supper_committee_create_update_committee_validator_proposal">create_update_committee_validator_proposal</a>(
    self: &<b>mut</b> <a href="supper_committee.md#0x3_supper_committee_SupperCommittee">SupperCommittee</a>,
    operate: bool,
    committee_validator: <b>address</b> ,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext
){
    <b>if</b>(operate){
        <b>assert</b>!(!self.committee_validators.contains(&committee_validator),<a href="supper_committee.md#0x3_supper_committee_ECommitteeValidatorAlreadyExists">ECommitteeValidatorAlreadyExists</a>);
    }<b>else</b> {
        <b>assert</b>!(self.committee_validators.contains(&committee_validator),<a href="supper_committee.md#0x3_supper_committee_ECommitteeValidatorNotExists">ECommitteeValidatorNotExists</a>);
    };
    self.<a href="supper_committee.md#0x3_supper_committee_create_proposal">create_proposal</a>(<a href="supper_committee.md#0x3_supper_committee_UpdateCommitteeValidatorAction">UpdateCommitteeValidatorAction</a>{operate,committee_validator}, <a href="../sui-framework/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0x3_supper_committee_execute_update_committee_validator_action"></a>

## Function `execute_update_committee_validator_action`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="supper_committee.md#0x3_supper_committee_execute_update_committee_validator_action">execute_update_committee_validator_action</a>(self: &<b>mut</b> <a href="supper_committee.md#0x3_supper_committee_SupperCommittee">supper_committee::SupperCommittee</a>, action: &<a href="supper_committee.md#0x3_supper_committee_UpdateCommitteeValidatorAction">supper_committee::UpdateCommitteeValidatorAction</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="supper_committee.md#0x3_supper_committee_execute_update_committee_validator_action">execute_update_committee_validator_action</a>(self:&<b>mut</b> <a href="supper_committee.md#0x3_supper_committee_SupperCommittee">SupperCommittee</a>,action: &<a href="supper_committee.md#0x3_supper_committee_UpdateCommitteeValidatorAction">UpdateCommitteeValidatorAction</a>){
    <b>if</b>(action.operate){
        <b>assert</b>!(!self.committee_validators.contains(&action.committee_validator),<a href="supper_committee.md#0x3_supper_committee_ECommitteeValidatorAlreadyExists">ECommitteeValidatorAlreadyExists</a>);
        self.committee_validators.insert(action.committee_validator)
    }<b>else</b> {
        <b>assert</b>!(self.committee_validators.contains(&action.committee_validator),<a href="supper_committee.md#0x3_supper_committee_ECommitteeValidatorNotExists">ECommitteeValidatorNotExists</a>);
        self.committee_validators.remove(&action.committee_validator)
    }
}
</code></pre>



</details>

<a name="0x3_supper_committee_vote_proposal"></a>

## Function `vote_proposal`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="supper_committee.md#0x3_supper_committee_vote_proposal">vote_proposal</a>(self: &<a href="supper_committee.md#0x3_supper_committee_SupperCommittee">supper_committee::SupperCommittee</a>, proposal: &<b>mut</b> <a href="supper_committee.md#0x3_supper_committee_Proposal">supper_committee::Proposal</a>, agree: bool, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="supper_committee.md#0x3_supper_committee_vote_proposal">vote_proposal</a>(
    self: &<a href="supper_committee.md#0x3_supper_committee_SupperCommittee">SupperCommittee</a>,
    proposal: &<b>mut</b> <a href="supper_committee.md#0x3_supper_committee_Proposal">Proposal</a>,
    agree: bool,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &TxContext,
){
    <b>let</b> sender = ctx.sender();
    <b>assert</b>!(self.committee_validators.contains(&sender),<a href="supper_committee.md#0x3_supper_committee_ENotExistsCommitteeAddress">ENotExistsCommitteeAddress</a>);

    <b>assert</b>!(proposal.<a href="supper_committee.md#0x3_supper_committee_proposal_status">proposal_status</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>) == <a href="supper_committee.md#0x3_supper_committee_PROPOSAl_STATUS_ACTIVE">PROPOSAl_STATUS_ACTIVE</a>,<a href="supper_committee.md#0x3_supper_committee_ENotProposalStatusProgress">ENotProposalStatusProgress</a>);

    <b>let</b> proportion =  <b>if</b> (proposal.action_type == <a href="../move-stdlib/type_name.md#0x1_type_name_get">type_name::get</a>&lt;<a href="supper_committee.md#0x3_supper_committee_UpdateCommitteeValidatorAction">UpdateCommitteeValidatorAction</a>&gt;().into_string()){
        self.committee_validators.size() * <a href="supper_committee.md#0x3_supper_committee_Supper_Committee_Quorum_Proportion">Supper_Committee_Quorum_Proportion</a> / <a href="supper_committee.md#0x3_supper_committee_Denominator">Denominator</a>
    }<b>else</b> {
        self.committee_validators.size() * <a href="supper_committee.md#0x3_supper_committee_Base_Quorum_Proportion">Base_Quorum_Proportion</a> / <a href="supper_committee.md#0x3_supper_committee_Denominator">Denominator</a>
    };

    <b>if</b>(agree){
        proposal.for_votes.insert(sender);
    }<b>else</b> {
        proposal.against_votes.insert(sender);
    };

    <b>if</b>(proposal.for_votes.size() &gt;= proportion){
        proposal.status = <a href="supper_committee.md#0x3_supper_committee_PROPOSAl_STATUS_PASS">PROPOSAl_STATUS_PASS</a>;
    }<b>else</b> <b>if</b> (proposal.against_votes.size() &gt; proportion){
        proposal.status = <a href="supper_committee.md#0x3_supper_committee_PROPOSAl_STATUS_FAIL">PROPOSAl_STATUS_FAIL</a>;
    };


    <b>let</b> vote_event = <a href="supper_committee.md#0x3_supper_committee_VoteProposalEvent">VoteProposalEvent</a>{
        proposal_id: <a href="../sui-framework/object.md#0x2_object_id">object::id</a>(proposal),
        voter: sender,
        agree,
        status: proposal.status
    };

    <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(vote_event);
}
</code></pre>



</details>

<a name="0x3_supper_committee_is_committee_validator"></a>

## Function `is_committee_validator`



<pre><code><b>public</b> <b>fun</b> <a href="supper_committee.md#0x3_supper_committee_is_committee_validator">is_committee_validator</a>(self: &<a href="supper_committee.md#0x3_supper_committee_SupperCommittee">supper_committee::SupperCommittee</a>, addr: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="supper_committee.md#0x3_supper_committee_is_committee_validator">is_committee_validator</a>(self: &<a href="supper_committee.md#0x3_supper_committee_SupperCommittee">SupperCommittee</a>,addr: <b>address</b>):bool{
    self.committee_validators.contains(&addr)
}
</code></pre>



</details>

<a name="0x3_supper_committee_proposal_status"></a>

## Function `proposal_status`



<pre><code><b>public</b> <b>fun</b> <a href="supper_committee.md#0x3_supper_committee_proposal_status">proposal_status</a>(self: &<a href="supper_committee.md#0x3_supper_committee_Proposal">supper_committee::Proposal</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="supper_committee.md#0x3_supper_committee_proposal_status">proposal_status</a>(self: &<a href="supper_committee.md#0x3_supper_committee_Proposal">Proposal</a>,<a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock):u8{
    <b>if</b>(self.start_time_ms &gt; <a href="../sui-framework/clock.md#0x2_clock">clock</a>.timestamp_ms()){
        <a href="supper_committee.md#0x3_supper_committee_PROPOSAl_STATUS_PENDING">PROPOSAl_STATUS_PENDING</a>
    }<b>else</b> <b>if</b>(self.status ==  <a href="supper_committee.md#0x3_supper_committee_PROPOSAl_STATUS_ACTIVE">PROPOSAl_STATUS_ACTIVE</a> && <a href="../sui-framework/clock.md#0x2_clock">clock</a>.timestamp_ms() &gt; self.end_time_ms){
        <a href="supper_committee.md#0x3_supper_committee_PROPOSAl_STATUS_TIMEOUT">PROPOSAl_STATUS_TIMEOUT</a>
    }<b>else</b> {
        self.status
    }
}
</code></pre>



</details>

<a name="0x3_supper_committee_proposal_action_type"></a>

## Function `proposal_action_type`



<pre><code><b>public</b> <b>fun</b> <a href="supper_committee.md#0x3_supper_committee_proposal_action_type">proposal_action_type</a>(self: &<a href="supper_committee.md#0x3_supper_committee_Proposal">supper_committee::Proposal</a>): <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="supper_committee.md#0x3_supper_committee_proposal_action_type">proposal_action_type</a>(self: &<a href="supper_committee.md#0x3_supper_committee_Proposal">Proposal</a>):String{
    self.action_type
}
</code></pre>



</details>

<a name="0x3_supper_committee_proposal_status_pass"></a>

## Function `proposal_status_pass`



<pre><code><b>public</b> <b>fun</b> <a href="supper_committee.md#0x3_supper_committee_proposal_status_pass">proposal_status_pass</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="supper_committee.md#0x3_supper_committee_proposal_status_pass">proposal_status_pass</a>():u8{
    <a href="supper_committee.md#0x3_supper_committee_PROPOSAl_STATUS_PASS">PROPOSAl_STATUS_PASS</a>
}
</code></pre>



</details>

<a name="0x3_supper_committee_action"></a>

## Function `action`



<pre><code><b>public</b> <b>fun</b> <a href="supper_committee.md#0x3_supper_committee_action">action</a>&lt;Action: store&gt;(self: &<a href="supper_committee.md#0x3_supper_committee_Proposal">supper_committee::Proposal</a>): &Action
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="supper_committee.md#0x3_supper_committee_action">action</a>&lt;Action:store&gt;(self: &<a href="supper_committee.md#0x3_supper_committee_Proposal">Proposal</a>):&Action{
    df::borrow&lt;<a href="supper_committee.md#0x3_supper_committee_ActionKey">ActionKey</a>,Action&gt;(&self.id, <a href="supper_committee.md#0x3_supper_committee_ActionKey">ActionKey</a>{})
}
</code></pre>



</details>

<a name="0x3_supper_committee_create_proposal"></a>

## Function `create_proposal`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="supper_committee.md#0x3_supper_committee_create_proposal">create_proposal</a>&lt;Action: store&gt;(self: &<b>mut</b> <a href="supper_committee.md#0x3_supper_committee_SupperCommittee">supper_committee::SupperCommittee</a>, action: Action, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="supper_committee.md#0x3_supper_committee_create_proposal">create_proposal</a>&lt;Action:store&gt;(
    self: &<b>mut</b> <a href="supper_committee.md#0x3_supper_committee_SupperCommittee">SupperCommittee</a>,
    action: Action,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext,
){
    <b>let</b> sender = ctx.sender();
    <b>assert</b>!(self.committee_validators.contains(&sender),<a href="supper_committee.md#0x3_supper_committee_ENotExistsCommitteeAddress">ENotExistsCommitteeAddress</a>);
    <b>let</b> action_type = <a href="../move-stdlib/type_name.md#0x1_type_name_get">type_name::get</a>&lt;Action&gt;();
    // only <a href="sui_system.md#0x3_sui_system">sui_system</a> action <b>struct</b> <a href="../sui-framework/types.md#0x2_types">types</a>
    <b>assert</b>!(action_type.get_address() == address::to_ascii_string(@0x3),<a href="supper_committee.md#0x3_supper_committee_ENotSupportStructType">ENotSupportStructType</a>);

    <b>let</b> <b>mut</b> proposal = <a href="supper_committee.md#0x3_supper_committee_Proposal">Proposal</a>{
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        proposer: sender,
        for_votes:<a href="../sui-framework/vec_set.md#0x2_vec_set_empty">vec_set::empty</a>(),
        against_votes:<a href="../sui-framework/vec_set.md#0x2_vec_set_empty">vec_set::empty</a>(),
        start_time_ms: <a href="../sui-framework/clock.md#0x2_clock">clock</a>.timestamp_ms(),
        end_time_ms:<a href="../sui-framework/clock.md#0x2_clock">clock</a>.timestamp_ms() + <a href="supper_committee.md#0x3_supper_committee_Timeout">Timeout</a>,
        status: <a href="supper_committee.md#0x3_supper_committee_PROPOSAl_STATUS_ACTIVE">PROPOSAl_STATUS_ACTIVE</a>,
        action_type: action_type.into_string(),
    };

    proposal.for_votes.insert(sender);

    <b>let</b> create_proposal_event = <a href="supper_committee.md#0x3_supper_committee_CreateProposalEvent">CreateProposalEvent</a>{
        proposal_id: <a href="../sui-framework/object.md#0x2_object_id">object::id</a>(&proposal),
        proposer: proposal.proposer,
        action_type: proposal.action_type
    };


    df::add(&<b>mut</b> proposal.id, <a href="supper_committee.md#0x3_supper_committee_ActionKey">ActionKey</a>{}, action);

    self.proposal_list.push_back(<a href="../sui-framework/object.md#0x2_object_id">object::id</a>(&proposal));

    <a href="../sui-framework/transfer.md#0x2_transfer_share_object">transfer::share_object</a>(proposal);

    <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(create_proposal_event);
}
</code></pre>



</details>
