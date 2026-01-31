create table group_memebers(
    group_id_ bigint primary key,
    member_id_ bigint not null unique,
    joined_at_ timestamptz
);