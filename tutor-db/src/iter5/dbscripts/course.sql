DROP TABLE IF EXISTS ezy_course_c6;

CREATE TABLE ezy_course_c6
(
    course_id SERIAL PRIMARY KEY,
    tutor_id INT NOT NULL,
    course_name VARCHAR(140) NOT NULL,
    course_description VARCHAR(2000),
    course_format VARCHAR(30),
    course_structure VARCHAR(200),
    course_duration VARCHAR(30),
    course_price INT,
    course_language VARCHAR(30),
    course_level VARCHAR(30),
    posted_time TIMESTAMP default now()
);

insert into ezy_course_c6
    (course_id, tutor_id, course_name, posted_time)
values(DEFAULT, 1, 'First course', '2020-12-17 05:40:00');
insert into ezy_course_c6
    (course_id, tutor_id, course_name, posted_time)
values(DEFAULT, 1, 'Second course', '2020-12-17 05:45:00');