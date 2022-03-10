from typing import List
import pygame
import math
import dataclasses
import json

def parseGameboard():
    gameboardFile =  open('gameboard.json')

    territoryMap = json.load(gameboardFile)
    print(territoryMap)
    
    territoryList = []
    for id in territoryMap['territory_map']:
        terrData = territoryMap['territory_map'][id]
        currTerritory = Territory(terrData['id'], terrData['num_dice'], terrData['owner_id'], terrData['neighbors'])
        territoryList.append(currTerritory)

    return territoryList   

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

territoryOne = Territory(0, 1, 1, [1, 2])
territoryTwo = Territory(3, 1, 0, [1])
territoryThree = Territory(1, 1, 1, [0, 2, 3])
territoryFour = Territory(2, 1, 0, [1, 0])

#territoryList = [territoryOne, territoryTwo, territoryThree, territoryFour]
territoryList = parseGameboard()

running = True
while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False

    screen.fill((255, 255, 255))

    rotation = 0.0
    delta_rotation = (2.0 * 3.14159) / len(territoryList)
    radius = 200.0

    x_origin = window_width / 2.0
    y_origin = window_height / 2.0
    pygame.draw.circle(screen, (0, 0, 0), (x_origin, y_origin), 10)

    for cur_terr in territoryList:
        x = (radius * math.sin(rotation)) + x_origin
        y = (radius * math.cos(rotation)) + y_origin
        pygame.draw.circle(screen, (0, 0, 255), (x, y), 50)

        terr_id_text_img = font.render(str(cur_terr.id), True, (0, 0, 0))
        screen.blit(terr_id_text_img, (x, y))

        rotation += delta_rotation

    pygame.display.flip()

pygame.quit()