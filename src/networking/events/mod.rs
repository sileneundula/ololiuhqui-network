/// LiuhqEvent is a struct that represents an event in the Liuhq network.
pub struct LiuhqEvent {
    pub event_type: String,
    pub event_data: String,
}


pub enum LiuhqEventType {
    /// Event for a new message
    NewMessage,
    /// Event for a new peer
    NewPeer,
    /// Event for a new connection
    NewConnection,
    /// Event for a new disconnection
    NewDisconnection,
    /// Event for a new error
    NewError,
    /// Event for a new request
    NewRequest,
    /// Event for a new response
    NewResponse,
    /// Event for a new notification
    NewNotification,
    /// Event for a new subscription
    NewSubscription,
    /// Event for a new unsubscription
    NewUnsubscription,
    /// Event for a new broadcast
    NewBroadcast,
    /// Event for a new unicast
    NewUnicast,
    /// Event for a new multicast
    NewMulticast,
    /// Event for a new group
    NewGroup,
    /// Event for a new group member
    NewGroupMember,
    /// Event for a new group message
    NewGroupMessage,
    /// Event for a new group request
    NewGroupRequest,
    /// Event for a new group response
    NewGroupResponse,
    /// Event for a new group notification
    NewGroupNotification,
    /// Event for a new group subscription
    NewGroupSubscription,
    /// Event for a new group unsubscription
    NewGroupUnsubscription,
    /// Event for a new group broadcast
    NewGroupBroadcast,
    /// Event for a new group unicast
    NewGroupUnicast,
    /// Event for a new group multicast
    NewGroupMulticast,
    /// Event for a new group member request
    NewGroupMemberRequest,
    /// Event for a new group member response
    NewGroupMemberResponse,
    /// Event for a new group member notification
    NewGroupMemberNotification,
}

pub enum LiuhqEvents {
    /// Event for a new message
    NewMessage(LiuhqEvent),
    /// Event for a new peer
    NewPeer(LiuhqEvent),
    /// Event for a new connection
    NewConnection(LiuhqEvent),
    /// Event for a new disconnection
    NewDisconnection(LiuhqEvent),
    /// Event for a new error
    NewError(LiuhqEvent),
    /// Event for a new request
    NewRequest(LiuhqEvent),
    /// Event for a new response
    NewResponse(LiuhqEvent),
    /// Event for a new notification
    NewNotification(LiuhqEvent),
    /// Event for a new subscription
    NewSubscription(LiuhqEvent),
    /// Event for a new unsubscription
    NewUnsubscription(LiuhqEvent),
    /// Event for a new broadcast
    NewBroadcast(LiuhqEvent),
    /// Event for a new unicast
    NewUnicast(LiuhqEvent),
    /// Event for a new multicast
    NewMulticast(LiuhqEvent),
    /// Event for a new group
    NewGroup(LiuhqEvent),
    /// Event for a new group member
    NewGroupMember(LiuhqEvent),
    /// Event for a new group message
    NewGroupMessage(LiuhqEvent),
    /// Event for a new group request
    NewGroupRequest(LiuhqEvent),
    /// Event for a new group response
    NewGroupResponse(LiuhqEvent),
    /// Event for a new group notification
    NewGroupNotification(LiuhqEvent),
    /// Event for a new group subscription
    NewGroupSubscription(LiuhqEvent),
    /// Event for a new group unsubscription
    NewGroupUnsubscription(LiuhqEvent),
    /// Event for a new group broadcast
    NewGroupBroadcast(LiuhqEvent),
    /// Event for a new group unicast
    NewGroupUnicast(LiuhqEvent),
    /// Event for a new group multicast
    NewGroupMulticast(LiuhqEvent),
    /// Event for a new group member request
    NewGroupMemberRequest(LiuhqEvent),
    /// Event for a new group member response
    NewGroupMemberResponse(LiuhqEvent),
    /// Event for a new group member notification
    NewGroup,
    MemberNotification(LiuhqEvent),
}