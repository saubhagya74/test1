create table home(
    notification_id_ bigint primary key,
    user_id_ bigint not null,
    notification_object_ jsonb default '{}'::jsonb,
    created_at_ timestamptz not null,
    is_seen_ boolean not null,
    is_deleted_ boolean not null
    --its is deleted there should be is to be deleted for those obj which has ttl
);
create index see_unread_notification_count_ 
on home (user_id_)
where is_seen_=false and is_deleted_ =false;
-- just for count though it works for select *, we need pagination too/
create index see_unread_notification_
on home (user_id_, created_at_ desc)
where is_seen_=false and is_deleted_ =false;
-- now after soclling too far you can display seen as well ,make a index for that too???
-- when seen we delete by 
-- vaccum aNanlyze request_;  to reorganize