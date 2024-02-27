FROM php:cli

ENV COMPOSER_ALLOW_SUPERUSER=1 \
  COMPOSER_HOME=/composer

COPY --from=composer:2.5 /usr/bin/composer /usr/bin/composer
