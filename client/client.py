from asyncio.windows_events import NULL
from typing import Dict, List
from xml.etree.ElementTree import QName
import pygame
import math
import dataclasses
import json
import socket
import sys
import threading

BLACK = (0, 0, 0)
RED = (235, 72, 55)

def parseGameboard(json_str):
    json_map = json.loads(json_str)
    
    territory_map: Dict[int, Territory] = {}
    for id_str in json_map['territory_map']:
        id = int(id_str)
        terr_data = json_map['territory_map'][id_str]
        curr_territory = Territory(terr_data['id'], terr_data['num_dice'], terr_data['owner_id'], terr_data['neighbors'])
        territory_map[id] = curr_territory

    return territory_map

def draw_gameboard():
    global game_state

    territory_map = game_state.territory_map

    screen.fill((255, 255, 255))

    rotation = 0.0
    delta_rotation = (2.0 * 3.14159) / len(territory_map)
    radius = 200.0

    x_origin = window_width / 2.0
    y_origin = window_height / 2.0
    pygame.draw.circle(screen, (0, 0, 0), (x_origin, y_origin), 10)

    coord_map = {}
    rect_map = {}

    for curr_id, cur_terr in territory_map.items():
        x = (radius * math.sin(rotation)) + x_origin
        y = (radius * math.cos(rotation)) + y_origin

        terr_color = (0, 0, 255)
        if cur_terr.owner_id !=  0:
            terr_color = (0, 255, 0)

        rect = pygame.draw.circle(screen, terr_color, (x, y), 50)
        
        rect_map[cur_terr.id] = rect
        coord_map[cur_terr.id] = (x,y)

        terr_id_text_img = font.render(str(cur_terr.id), True, (0, 0, 0))
        screen.blit(terr_id_text_img, (x, y))

        terr_id_text_img = font.render(str(cur_terr.num_dice), True, (255, 255, 255))
        screen.blit(terr_id_text_img, (x, y+10))

        rotation += delta_rotation

    for curr_id, cur_terr in territory_map.items():
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

    game_state.rect_map = rect_map

def handle_click():
    global game_state

    (x_pos, y_pos) = pygame.mouse.get_pos()
    print('mouse clicked ', x_pos, y_pos)

    for cur_terr in game_state.rect_map:
        if game_state.rect_map[cur_terr].collidepoint(x_pos, y_pos):
            handle_territory_selected(cur_terr)
            print("Clicked inside territory", cur_terr)

    global attack_rect
    if attack_rect.collidepoint(x_pos, y_pos):
            print("Clicked \"Attack\"")
            handle_attack()

def handle_territory_selected(clicked_territory):
    global game_state

    if game_state.attack_from < 0:
        game_state.attack_from = clicked_territory
    elif game_state.attack_from >= 0 and game_state.attack_to < 0:
        game_state.attack_to = clicked_territory
    elif game_state.attack_from >= 0 and game_state.attack_to >= 0:
        game_state.attack_from = clicked_territory
        game_state.attack_to = -1

def handle_attack():
    global game_state
    can_attack = True
    print(game_state.territory_map)
    
    if game_state.attack_from < 0 or game_state.attack_to < 0:
        can_attack = False
        print("Cannot attack: need to select two territories")

    elif game_state.territory_map[game_state.attack_from].owner_id == game_state.territory_map[game_state.attack_to].owner_id:
        can_attack = False
        print("Cannot attack: need to select two territories belonging to two separate players")

    elif game_state.attack_from not in game_state.territory_map[game_state.attack_to].neighbors:
        can_attack = False
        print("Cannot attack: need to select two territories adjacent to each other")
    
    if can_attack:
        send_message('Attack;' + str(game_state.attack_from) + ';' + str(game_state.attack_to))


def send_message(message):
    global socket
    try:
        print('sending {0}'.format(message))
        num_sent = socket.send(message.encode('utf-8'))
        print("Sent: ", num_sent)
    except BaseException as err:
        print("Error sending: {0}".format(err))

def socket_recv():
    global socket
    global running

    while running:
        wait_for_and_process_message()    

def wait_for_and_process_message():
    print("Recv socket waiting...")
    message = socket.recv(2048)
    parse_message(message)

def parse_message(message):
    global game_state

    decoded_msg = message.decode("utf-8")
    print(decoded_msg)
    split_msg = decoded_msg.split(";")
    
    if len(split_msg) > 1:
        match split_msg[0]:
            case "Gameboard":
                game_state.territory_map = parseGameboard(split_msg[1])
                draw_gameboard()
    



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
    territory_map: Dict[int, Territory]
    rect_map: Dict[int, pygame.Rect]

pygame.init()

window_width = 800
window_height = 600
screen = pygame.display.set_mode([window_width, window_height])

font = pygame.font.SysFont(None, 16)

territory_map = {}
rect_map = {}
game_state = GameState(-1, -1, territory_map, rect_map)
running = True

socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
socket.connect(("localhost", 1234))

send_message('Connect')
wait_for_and_process_message()

socket_thread = threading.Thread(target=socket_recv, args=())
socket_thread.start()
#raw_gameboard = socket.recv(2048)
#json_gameboard = raw_gameboard.decode("utf-8")
#territory_map = parseGameboard(json_gameboard)

attack_rect = NULL

while running:

    draw_gameboard()

    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False
        if event.type == pygame.MOUSEBUTTONUP:
            handle_click()

socket.close()
socket_thread.join()
pygame.quit()