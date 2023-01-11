-- Add migration script here
INSERT INTO
  users (id, username, password_hash)
VALUES
  (
    'ef9379da-2675-458d-ac9e-9787323cd87a',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$xgxS/L7Ii3qyGw0C53qyqA$6h6CxH0F6xNOT/xn4DSMMsM39YKlFr81y77WMNgQrWM'
  );
