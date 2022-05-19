from asyncio.windows_events import NULL
from typing import List
import pygame
import math
import dataclasses
import json
import socket
import sys

BLACK = (0, 0, 0)
RED = (235, 72, 55)

def parseGameboard(json_str):
    territory_map = json.loads(json_str)
    
    territory_list = []
    for id in territory_map['territory_map']:
        terr_data = territory_map['territory_map'][id]
        curr_territory = Territory(terr_data['id'], terr_data['num_dice'], terr_data['owner_id'], terr_data['neighbors'])
        territory_list.append(curr_territory)

    return territory_list

def draw_gameboard(territory_list):
    screen.fill((255, 255, 255))

    rotation = 0.0
    delta_rotation = (2.0 * 3.14159) / len(territory_list)
    radius = 200.0

    x_origin = window_width / 2.0
    y_origin = window_height / 2.0
    pygame.draw.circle(screen, (0, 0, 0), (x_origin, y_origin), 10)

    coord_map = {}
    rect_map = {}

    for cur_terr in territory_list:
        x = (radius * math.sin(rotation)) + x_origin
        y = (radius * math.cos(rotation)) + y_origin
        rect = pygame.draw.circle(screen, (0, 0, 255), (x, y), 50)
        
        rect_map[cur_terr.id] = rect
        coord_map[cur_terr.id] = (x,y)

        terr_id_text_img = font.render(str(cur_terr.id), True, (0, 0, 0))
        screen.blit(terr_id_text_img, (x, y))

        rotation += delta_rotation

    for cur_terr in territory_list:
        for cur_neighbor in cur_terr.neighbors:
            pygame.draw.line(screen, (255, 0, 0), coord_map[cur_terr.id], coord_map[cur_neighbor])


    global BLACK
    attack_button_color = BLACK
    if game_state.attack_from >= 0 and game_state.attack_to >= 0:
        pygame.draw.line(screen, (0, 255, 0), coord_map[game_state.attack_from], coord_map[game_state.attack_to])
        global RED
        attack_button_color = RED

    global attack_rect
    attack_rect = pygame.draw.circle(screen, attack_button_color, (51, 51), 50)
    attack_text_img = font.render("Attack", True, (255, 255, 255), (0,0,0))
    screen.blit(attack_text_img, (46, 41))

    pygame.display.flip()

    return rect_map

def handle_click(game_state, rect_map):
    (x_pos, y_pos) = pygame.mouse.get_pos()
    print('mouse clicked ', x_pos, y_pos)

    for cur_terr in rect_map:
        if rect_map[cur_terr].collidepoint(x_pos, y_pos):
            handle_attack(game_state, cur_terr)
            print("Clicked inside territory", cur_terr)

    global attack_rect
    if attack_rect.collidepoint(x_pos, y_pos):
            print("Clicked \"Attack\"")

def handle_attack(game_state, clicked_territory):
    if game_state.attack_from < 0:
        game_state.attack_from = clicked_territory
    elif game_state.attack_from >= 0 and game_state.attack_to < 0:
        game_state.attack_to = clicked_territory
    elif game_state.attack_from >= 0 and game_state.attack_to >= 0:
        game_state.attack_from = clicked_territory
        game_state.attack_to = -1


@dataclasses.dataclass
class Territory:
    id: int
    num_dice: int
    owner_id: int
    neighbors: List

@dataclasses.dataclass
class GameState:
    attack_to: int
    attack_from: int

pygame.init()

window_width = 800
window_height = 600
screen = pygame.display.set_mode([window_width, window_height])

font = pygame.font.SysFont(None, 16)

socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
socket.connect(("localhost", 1234))
msg = 'Connect'.encode('utf-8')
print('sending {0}'.format(msg))

try:
    num_sent = socket.send(msg)
    print("Sent: ", num_sent)
except BaseException as err:
    print("Error sending: {0}".format(err))

raw_gameboard = socket.recv(2048)
json_gameboard = raw_gameboard.decode("utf-8")
territory_list = parseGameboard(json_gameboard)

game_state = GameState(-1, -1)

attack_rect = NULL

running = True
while running:

    rect_map = draw_gameboard(territory_list)

    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False
        if event.type == pygame.MOUSEBUTTONUP:
            handle_click(game_state, rect_map)

    
socket.close()
pygame.quit()