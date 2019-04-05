-- Your SQL goes here

CREATE VIEW archives(cnt, yer, mon) AS 
SELECT DISTINCT COUNT(*), EXTRACT(year from publish_time) AS YEAR, EXTRACT(month from publish_time) AS MONTH FROM posts
GROUP BY date_part('year',publish_time), date_part('month', publish_time);