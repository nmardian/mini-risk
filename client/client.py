import pygame
import math
pygame.init()

window_width = 800
window_height = 600
screen = pygame.display.set_mode([window_width, window_height])

font = pygame.font.SysFont(None, 16)

running = True
while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False

    screen.fill((255, 255, 255))

    num_terrs = 4
    rotation = 0.0
    delta_rotation = (2.0 * 3.14159) / num_terrs
    radius = 200.0

    x_origin = window_width / 2.0
    y_origin = window_height / 2.0
    pygame.draw.circle(screen, (0, 0, 0), (x_origin, y_origin), 10)

    for cur_terr in range(0,num_terrs):
        x = (radius * math.sin(rotation)) + x_origin
        y = (radius * math.cos(rotation)) + y_origin
        pygame.draw.circle(screen, (0, 0, 255), (x, y), 50)

        terr_id_text_img = font.render(str(cur_terr), True, (0, 0, 0))
        screen.blit(terr_id_text_img, (x, y))

        rotation += delta_rotation

    pygame.display.flip()

pygame.quit()