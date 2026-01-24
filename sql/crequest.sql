create table request_(
    request_id_ bigint primary key,
    sender_id_ bigint not null,
    receiver_id_ bigint not null,
    request_status boolean not null,
    -- 1 pending 0 delete
    requested_at timestamptz not null,
)
create unique index prevent_duplicate_id 
on request_ (sender_id_ , receiver_id_);
-- make index for to see received request and req sent
-- create type request_label as enum ('pending','accepted','declined');
-- expect pending delete vayesi just use boolean
--push the accepted whole json to the sender home row