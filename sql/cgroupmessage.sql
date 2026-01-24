create table messages_(
    message_id_ bigint primary key,
    chat_id_ bigint not null,
    sender_id_ bigint not null,
    content_type_ content_label_,
    description_ varchar(2000),
    messeged_at_ timestamptz not null,
    compression_type_ compression_label_ ,
    encryption_type_ encryption_label_,
    reaction_id_ bigint,
    is_edited_ boolean,
    is_deleted boolean
)