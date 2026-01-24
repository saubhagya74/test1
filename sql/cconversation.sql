-- conversation

-- - chatid
-- - chatname
-- - last message
-- - last time
-- - user a id
-- - user b id
-- - conversation settings:block, restricted, restricted+deleted????
-- - mute? mute direction?
create table conversation_(
    conversation_id_ bigint primary key,
    chat_name_ varchar(30),
    last_message_ varchar(15) not null,
    -- //update this not null,need it, maybe??or not?
    last_time_ timestamptz default CURRENT_TIMESTAMP,
    user_a_id_ bigint not null,
    user_b_id_ bigint not null,
    settings_ jsonb default '{
    "is_pinned_":"false",
    "notification_level_":"all",
    "theme_":"default",
    "direction_": [0,0]
    }'::jsonb
);
-- a is 1 and b is -1 
-- create table conversation_setings_(

-- )
-- For "Celebrity" accounts (like a user with 1M followers), the first 
-- few pages of the follower list are usually stored 
-- in Redis (an in-memory cache) so the database isn't even touched.
-- for mutual folower we do intersect