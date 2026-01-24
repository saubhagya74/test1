create table group(
    group_id_ bigint primary key,
    last_message_ varchar(20),
    -- attach meta data of grup creation and memenbers adding in a message
    last_time_ timestamptz,
    created_at_ timestamptz not null,
    profile_url_ varchar(300),
    group_settings_ jsonb default "{}"::jsonb
-- k rakhne?> per user setting ko lagi diff table?
)