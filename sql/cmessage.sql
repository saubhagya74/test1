-- create type content_label as enum ("text","video");
create table messages_(
    message_id_ bigint primary key,
    chat_id_ bigint not null,
    sender_id_ bigint not null,
    receiver_id_ bigint not null,
    content_type_ content_label,
    description_ varchar(2000),
    messaged_at_ timestamptz not null,
    compression_type_ compression_label_ ,
    encryption_type_ encryption_label_,
    reaction_id_ smallint,
    is_edited_ boolean,
    is_deleted_ boolean
)
create index get_message_by_time_chat_id
on messages_ (chat_id_, messeged_at_ DESC)
where is_deleted = false;
-- isdeleted and edited along with reactins put in cold storage ft //future job
create type content_label as enum ('text','video','audio');
create type encryption_type as enum ('ecc','rsa');
create type compression_type as enum ('lz4','gzip','zstd');
\dT+ to list all
SELECT enum_range(NULL::encryption_label_);
-- create index get_recent_messages
-- on messages_ (sender_id_, receiver_id_,messeged_at_ desc)
-- where is_deleted_ = false vice versa too, not need this index
