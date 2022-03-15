from typing import List
import pygame
import math
import dataclasses
import json

def parseGameboard():
    gameboard_file =  open('gameboard.json')

    territory_map = json.load(gameboard_file)
    print(territory_map)
    
    territory_list = []
    for id in territory_map['territory_map']:
        terr_data = territory_map['territory_map'][id]
        curr_territory = Territory(terr_data['id'], terr_data['num_dice'], terr_data['owner_id'], terr_data['neighbors'])
        territory_list.append(curr_territory)

    return territory_list   

@dataclasses.dataclass
class Territory:
    id: int
    num_dice: int
    owner_id: int
    neighbors: List


pygame.init()

window_width = 800
window_height = 600
screen = pygame.display.set_mode([window_width, window_height])

font = pygame.font.SysFont(None, 16)

territory_list = parseGameboard()

running = True
while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False

    screen.fill((255, 255, 255))

    rotation = 0.0
    delta_rotation = (2.0 * 3.14159) / len(territory_list)
    radius = 200.0

    x_origin = window_width / 2.0
    y_origin = window_height / 2.0
    pygame.draw.circle(screen, (0, 0, 0), (x_origin, y_origin), 10)

    coord_map = {}

    for cur_terr in territory_list:
        x = (radius * math.sin(rotation)) + x_origin
        y = (radius * math.cos(rotation)) + y_origin
        pygame.draw.circle(screen, (0, 0, 255), (x, y), 50)
        coord_map[cur_terr.id] = (x,y)

        terr_id_text_img = font.render(str(cur_terr.id), True, (0, 0, 0))
        screen.blit(terr_id_text_img, (x, y))

        rotation += delta_rotation

    for cur_terr in territory_list:
        for cur_neighbor in cur_terr.neighbors:
            pygame.draw.line(screen, (255, 0, 0), coord_map[cur_terr.id], coord_map[cur_neighbor])

    pygame.display.flip()

pygame.quit()