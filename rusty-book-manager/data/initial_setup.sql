INSERT INTO
    roles (name)
VALUES
    ('Admin'),
    ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
    users (name, email, password_hash, role_id)
SELECT
    'Eleazar Fig',
    'eleazar.fig@example.com',
    '$2b$12$YLZ0BL3cE0HJZY45g0FOL.Z9VhVr/6.5F6.ylSkqsobb40NM1Oxme',
    role_id
FROM
    roles
WHERE
    name LIKE 'Admin';
