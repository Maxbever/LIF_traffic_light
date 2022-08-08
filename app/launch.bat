for /l %%x in (1, 1, 12) do (start /d D:\Documents\GitHub\LIF_traffic_light\app traffic_light.exe %%x)

for /l %%x in (13, 1, 15) do (start /d D:\Documents\GitHub\LIF_traffic_light\app first_layer.exe %%x)

start /d D:\Documents\GitHub\LIF_traffic_light\app second_layer.exe 16