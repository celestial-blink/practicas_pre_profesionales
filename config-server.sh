sudo mkdir -p /srv/practicasperupro/logs

sudo touch /srv/practicasperupro/logs/app.log

sudo chown -Rubuntu:ubuntu /srv/practicasperupro/logs

# nueve practicasperupro a logrotate.d
cp ./practicasperupro /etc/logrotate.d/practicasperupro
