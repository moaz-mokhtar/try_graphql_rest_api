-- Your SQL goes here
insert into
    users
values
    ('1', 'ahmed', 'pass'),
    ('2', 'baher', 'pass'),
    ('3', 'said', 'pass');

insert into
    team
values
    ('T1', 'partner'),
    ('T2', 'employee');

insert into
    member
values
    ('M1', '1', 'T1'),
    ('M2', '2', 'T2'),
    ('M3', '3', 'T2');