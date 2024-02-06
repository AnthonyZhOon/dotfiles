/*
Databases Applied 10
applied10_sql_advanced.sql

student id: 33109877
student name: Anthony Oon
last modified date: Git knows
*/
--1
-- Time is ambiguous, I assume the question demands the oldest student who has enrolled in
-- FIT9132

SELECT
    e.stuid,
    stufname
    || ' '
    || stulname AS stuname
FROM
    uni.student   s
    JOIN uni.enrolment e
    ON s.stuid=e.stuid
WHERE
    upper(e.unitcode) = upper('FIT9132')
    AND s.studob = ( (
        SELECT
            MIN(s.studob)
        FROM
            uni.student   s
            JOIN uni.enrolment e
            ON s.stuid = e.stuid
        WHERE
            lower(e.unitcode) = lower('FIT9132')
    ))
ORDER BY
    studob;

SELECT
    stuid
FROM
    uni.enrolment
WHERE
    lower(unitcode) = lower('FIT9132');

select o.unitcode, o.ofsemester, COUNT(*) as number_enrolments from
uni.offering o join uni.enrolment e on o.ofsemester = e.ofsemester
where extract(year from o.ofyear) = 2019
group by o.ofyear, o.ofsemester, o.unitcode
order by o.ofsemester, o.unitcode

--2
select * from uni.enrolment e join uni.student s
where e.unitcode = i;

-- Query the unitcode of the highest enrolled unit in 2019

select unitcode, count(stuid) as num_enrol from uni.enrolment
where extract(year from ofyear) = 2019
group by unitcode
having num_enrol = MAX();

--3



--4



--5



--6



--7
/* Using outer join */



/* Using set operator MINUS */



/* Using subquery */



--8



--9



--10



--11
-- Given that the payment rate for a tutorial is $42.85 per hour and the payment rate for a lecture
-- is $75.60 per hour, calculate the weekly payment per type of class for each staff member in
-- semester 1 2020. In the display, include staff id, staff name, type of class (lecture - L or
-- tutorial - T), number of classes, number of hours (total duration), and weekly payment (number of
--hours * payment rate). The weekly payment must be displayed to two decimal points and right aligned.
--Order the list by the staff id and for a given staff id by the type of class.
SELECT
    staffid,
    stafffname
    || ' '
    || stafflname                                        AS staffname,
    'Tutorial'                                           AS type,
    COUNT(*)                                             AS number_of_classes,
    SUM(clduration)                                      AS total_hours,
    lpad(to_char(SUM(clduration) * 42.8, '$990.99'), 14) AS weekly_pay
FROM
    uni.staff
    NATURAL JOIN uni.schedclass
WHERE
    ofsemester = 1
    AND EXTRACT(YEAR FROM ofyear) = 2020
    AND cltype = 'T'
GROUP BY
    staffid,
    stafffname,
    stafflname
UNION
SELECT
    staffid,
    stafffname
    || ' '
    || stafflname                                        AS staffname,
    'Lecture'                                            AS type,
    COUNT(*)                                             AS number_of_classes,
    SUM(clduration)                                      AS total_hours,
    lpad(to_char(SUM(clduration) * 75.6, '$990.99'), 14) AS weekly_pay
FROM
    uni.staff
    NATURAL JOIN uni.schedclass
WHERE
    ofsemester = 1
    AND EXTRACT(YEAR FROM ofyear) = 2020
    AND cltype = 'L'
GROUP BY
    staffid,
    stafffname,
    stafflname
ORDER BY
    staffid,
    type;

SELECT
    staffid,
    stafffname
    || ' '
    || stafflname            AS staffname,
    CASE cltype
        WHEN 'T' THEN
            'Tutorial'
        WHEN 'L' THEN
            'Lecutre'
        ELSE
            'Unknown'
    END                      AS type,
    COUNT(*)                 AS number_of_classes,
    SUM(clduration)          AS total_hours,
    lpad(to_char(SUM(clduration) *
        CASE cltype
            WHEN 'T' THEN
                42.8
            WHEN 'L' THEN
                75.6
            ELSE
                0
        END, '$990.99'), 14) AS weekly_pay
FROM
    uni.staff
    NATURAL JOIN uni.schedclass
WHERE
    ofsemester = 1
    AND EXTRACT(YEAR FROM ofyear) = 2020
GROUP BY
    staffid,
    stafffname,
    stafflname,
    cltype
ORDER BY
    staffid;

--12



--13
