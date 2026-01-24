create table reactions_(
    message_id_ bigint primary key,
    user_id_ bigint not null,
    emoji_id_ varchar(10) not null,
    reacted_at timestamptz not null
)