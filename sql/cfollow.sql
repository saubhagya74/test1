create table followed_following(
    follower_id_ bigint primary key,
    following_id_ bigint not null,
    followed_at timestamptz
    -- default for followed at?
)
