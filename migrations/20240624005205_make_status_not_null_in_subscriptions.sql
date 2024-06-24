-- Make `status` NOT NULL in `subscriptions` table

-- Wrap the whole migration in a transaction
-- to make sure it succeeds or fails atomically.
BEGIN;
    -- Backfill `status` for historical enteries
    UPDATE subscriptions
        SET status = 'confirmed'
        WHERE status IS NULL;
    -- Make `status` a mandatory column
    ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;