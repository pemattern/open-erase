INSERT INTO users (
    uuid,
    name,
    password_hash,
    created_at,
    modified_at
) VALUES (
    gen_random_uuid(),
    'system',
    -- password: secret
    '$argon2id$v=19$m=19456,t=2,p=1$c2FsdHltY3NhbHRzdGVy$AEBiLPbgT23PDqzPWAzKakRLOGlb+DaHevR0mzzMA/8',
    NOW(),
    NOW()
);

