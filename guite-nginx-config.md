sudo cp nginx-config /etc/nginx/sites-available/practicasperupro
sudo ln -s /etc/nginx/sites-available/practicasperupro /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
