-- Add down migration script here
ALTER TABLE public.transactions DROP COLUMN created_at;
ALTER TABLE public.internal_transactions DROP COLUMN created_at;
