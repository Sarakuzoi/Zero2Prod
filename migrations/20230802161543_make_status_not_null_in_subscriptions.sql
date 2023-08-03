-- We wrap the while migration in a transaction to make sure
-- it succeeds or fails atomically. 
BEGIN;
    -- Backfill `status` for historical entities
    UPDATE subscriptions
        SET status = 'confirmed'
        WHERE status IS NULL;
    -- Make `status` mandatory
    ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;