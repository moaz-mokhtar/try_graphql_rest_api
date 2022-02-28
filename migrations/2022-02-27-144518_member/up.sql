-- Your SQL goes here
create table member(
    id varchar primary key,
    user_id varchar not null,
    team_id varchar not null,
    constraint fk_user foreign key(user_id) references users(id) on delete cascade,
    constraint fk_team foreign key(team_id) references team(id) on delete cascade
)