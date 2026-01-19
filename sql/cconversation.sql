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
    last_message_ varchar(15),
    last_time_ timestamptz,
    user_a_id_ bigint,
    user_b_id_ bigint
);