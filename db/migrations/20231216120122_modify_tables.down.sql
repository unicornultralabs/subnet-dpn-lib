-- Add down migration script here
ALTER TABLE public.user_bandwidth_price DROP COLUMN rate_per_second;
ALTER TABLE public.user_bandwidth_price RENAME COLUMN rate_per_kb TO bandwidth_usage;
ALTER TABLE public.user_bandwidth_price ALTER COLUMN bandwidth_usage TYPE float8 USING bandwidth_usage::float8;
ALTER TABLE public.user_bandwidth_price ALTER COLUMN bandwidth_usage DROP DEFAULT;
