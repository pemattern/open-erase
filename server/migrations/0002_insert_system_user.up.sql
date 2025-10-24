INSERT INTO users (
    email,
    password_hash
) VALUES (
    'system',
    -- password: secret
    '$argon2id$v=19$m=19456,t=2,p=1$c2FsdHltY3NhbHRzdGVy$AEBiLPbgT23PDqzPWAzKakRLOGlb+DaHevR0mzzMA/8'
);

