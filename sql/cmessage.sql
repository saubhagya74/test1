-- create type content_label as enum ("text","video");
create table messages_(
    message_id_ bigint primary key,
    chat_id_ bigint,
    sender_id_ bigint,
    content_type_ varchar(10),
    description_ varchar(2000),
    encoding_type_ text,
    encryption_type_ text,
    reaction_id_ bigint,
    is_edited_ boolean,
    is_deleted boolean
)