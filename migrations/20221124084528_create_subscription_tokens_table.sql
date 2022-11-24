-- Add migration script here
CREATE TABLE
    subscription_tokens (
        token TEXT NOT NULL,
        subscriber_id UUID NOT NULL REFERENCES subscriptions (id),
        PRIMARY KEY (token)
    );
