create table users_(
    user_id_ bigint primary key,
    username_ varchar(25) not null,
    email_ varchar(60),
    password_hash_ Text
);