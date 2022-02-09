drop table if exists ezy_course_c5;

create table ezy_course_c5
(
    course_id serial primary key,
    tutor_id INT not null,
    course_name varchar(140) not null,
    posted_time TIMESTAMP default now()
);

insert into ezy_course_c5
    (course_id, tutor_id, course_name, posted_time)
values(DEFAULT, 1, 'First course', '2020-12-17 05:40:00');
insert into ezy_course_c
    (course_id, tutor_id, course_name, posted_time)
values(DEFAULT, 1, 'Second course', '2020-12-17 05:45:00');