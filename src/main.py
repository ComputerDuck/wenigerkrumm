import timeit
import math


def assign_globals(datei_name):
    """
    Liest die Datei mit einem gegebenem Datei_namen ein.
    :param datei_name: Name der einzulesenden Datei
    :return: nichts, da nur die global Variablen einen Wert zugewiesen bekommen.
    """
    array = []
    with open(datei_name) as tf:
        lines = tf.read()
        lines = lines.split("\n")
        lines.pop()
        for i, value in enumerate(lines):
            temp2 = lines[i].split(" ")
            temp = []
            for j in temp2:
                temp.append(float(j))
            temp.append(i)
            array.append(temp)
    global original
    original = array
    global amnt_pts
    amnt_pts = len(original)
    global dist_matrix
    dist_matrix = [[distance(original[i], original[j]) for j in range(amnt_pts)] for i in range(amnt_pts)]


def distance(point1, point2):
    """
    Abstand zwischen zwei Punkten wird berechnet.
    :param point1: Punkt 1
    :param point2: Punkt 2
    :return: der Abstand zwischen point1 und point2
    """
    vec = [point2[0] - point1[0], point2[1] - point1[1]]
    return math.sqrt(vec[0] ** 2 + vec[1] ** 2)


def length_of_route(route):
    """
    Berechnet die Länge einer Route.
    :param route: die Route, deren Länge berechnet werden soll.
    :return: Länge der Route.
    """
    return sum(dist_matrix[route[i - 1][2]][route[i][2]] for i in range(1, len(route)))


def nearest_neighbor(current_p, unvisited):
    """
    Berechnet iterativ den nächsten Punkt an current_p der noch nicht besucht wurde
    :param current_p: der aktuelle Punkt.
    :param unvisited: Liste an unbesuchten Punkten.
    :return: der von current_p aus nächstgelegene Punkt
    """
    ind = current_p[2]
    point = unvisited[0]
    nearest_dist = dist_matrix[ind][point[2]]
    for i in range(1, len(unvisited)):
        next_point = unvisited[i]
        next_dist = dist_matrix[ind][next_point[2]]
        if next_dist < nearest_dist:
            point = next_point
            nearest_dist = next_dist
    return point


def nn_possible(prev, current, unvisited):
    """
    Berechnet den nächsten Punkt von current, der besuchbar ist
    :param prev: vorheriger Punkt.
    :param current: aktueller Punkt.
    :param unvisited: Liste an unbesuchten Punkten.
    :return: Nächster besuchbarer Punkt an current.
    """
    vek1 = [current[0] - prev[0], current[1] - prev[1]]
    ind = current[2]
    point = None
    nearest_dist = float('inf')
    for i in range(len(unvisited)):
        vek2 = (unvisited[i][0] - current[0], unvisited[i][1] - current[1])
        if vek1[0] * vek2[0] + vek1[1] * vek2[1] >= 0 and dist_matrix[ind][unvisited[i][2]] < nearest_dist:
            point = unvisited[i]
            nearest_dist = dist_matrix[ind][point[2]]
    return point


def is_correct_angle(prev, current, next_p):
    """
    Kontrolliert, ob der Winkel kleiner oder gleich 90° ist.
    :param prev: vorheriger Punkt.
    :param current: aktueller Punkt.
    :param next_p: potentieller nächster Punkt
    :return: Ob der next_p besuchbar ist.
    """
    prev_vec = (current[0] - prev[0], current[1] - prev[1])
    next_vec = (next_p[0] - current[0], next_p[1] - current[1])
    skalar = prev_vec[0] * next_vec[0] + prev_vec[1] * next_vec[1]
    return skalar >= 0


def priority_queue(prev, current, unvisited):
    """
    Erstellung der priority Queue.
    :param prev: vorheriger Punkt.
    :param current: aktueller Punkt.
    :param unvisited: Liste an unbesuchten Punkten.
    :return: Die priority Queue.
    """
    unv = unvisited.copy()
    queue = []
    nn_p = nn_possible(prev, current, unvisited)
    if nn_p is None:
        return queue
    queue.append(nn_p)
    unv.remove(nn_p)

    while len(unv) > 0:
        temp = unv.pop()
        if is_correct_angle(prev, current, temp):
            for i, value in enumerate(queue):
                if dist_matrix[current[2]][value[2]] > dist_matrix[current[2]][temp[2]]:
                    queue.insert(i, temp)
                    break
            else:
                queue.append(temp)
    return queue


def start_route(start_p=0, second=-1):
    """
    Testet alle möglichen Wege, bis eine Route gefunden wurde.
    :param start_p: aktueller Startpunkt
    :param second: aktueller zweiter Punkt
    :return: Die Route.
    """
    if second >= amnt_pts > start_p:
        second = 0
        start_p += 1
    elif second < amnt_pts:
        second += 1
    if start_p >= amnt_pts:
        return []

    unvisited = original.copy()
    route = [original[start_p]]

    unvisited.pop(start_p)
    next_p = unvisited[0]
    route.append(next_p)
    unvisited.remove(next_p)
    possible, way = find_route_rek(route, unvisited)
    if possible:
        return way
    else:
        return start_route(start_p, second)


def find_route_rek(route, unvisited):
    """
    Berechnet rekursiv eine Route, die alle unbesuchten Punkte besucht, und keinen Abbiegewinkel hat der größer 90° ist.
    :param route: Die bisher besuchten Außenposten in der richtigen Reihenfolge.
    :param unvisited: Die unbesuchten Außenposten.
    :return: Ob eine Route möglich ist und die Route.
    """
    # Falls alle Punkte besucht werden ist die Route fertig
    if len(unvisited) == 0:
        return True, route

    # Eine nach der Nähe sortierte Liste der besuchbaren Außenposten
    pq = priority_queue(route[len(route) - 2], route[len(route) - 1], unvisited.copy())
    # Alle besuchbaren Punkte nacheinander testen
    for nearest_count, value in enumerate(pq):
        try_route = route.copy()
        try_unvisited = unvisited.copy()
        try_route.append(value)
        try_unvisited.remove(value)
        possible, way = find_route_rek(try_route, try_unvisited)
        if possible:
            return True, way

    return False, None


def normal_route():
    """
    berechnet eine Route mit dem normalen nearest neighbour Algorithmus
    :return: eine nearest neighbour Route ohne Einschränkungen
    """
    unvisited = original.copy()
    route = [unvisited.pop()]
    while len(unvisited) > 0:
        nn = nearest_neighbor(route[len(route) - 1], unvisited)
        route.append(nn)
        unvisited.remove(nn)
    return route


def start():
    """
    Diese Methode gibt nichts zurück und lässt den Nutzer auswählen für welche Datei er eine Route berechnet haben will.
    :return:
    """
    print("Falls das Programm gestoppt werden soll stopp eingeben.")
    while True:
        name_datei = "wenigerkrumm" + input("Welche Datei von 1-11 wollen Sie lösen lassen?: ") + ".txt"
        if name_datei.lower() == "wenigerkrummstopp.txt":
            break
        try:
            assign_globals(name_datei)
        except FileNotFoundError:
            print("Keine valide Angabe.")
            print("Deshalb wird nun mit der ersten Datei gerechnet.")
            assign_globals("wenigerkrumm1.txt")
        st = timeit.default_timer()
        if amnt_pts <= 0:
            print("Keine Punkte in der Datei enthalten.")
            return
        route = start_route()

        if len(route) > 0:
            end = timeit.default_timer()
            print(f"Es wurde in {round(end - st, 5)} Sekunden eine {length_of_route(route)}km lange Route gefunden.")
            print_r = input("Wollen Sie die Route sehen?(Ja oder Nein eintippen): ")
            if print_r.lower() == "ja":
                for i, value in enumerate(route):
                    print(f"Koordinaten des {i + 1}. Außenposten: x:{value[0]}, y:{value[1]}")
            elif print_r.lower() == "stopp":
                break
        else:
            print("Es ist keine Route ohne Abbiegewinkel, die größer als 90° sind möglich.")
            normal_r = input("Wollen Sie eine Route ohne Winkelbeschränkungen?(Ja oder Nein eintippen): ")
            if normal_r.lower() == "ja":
                st = timeit.default_timer()
                nr = normal_route()
                end = timeit.default_timer()
                print(f"In {end-st} wurde folgende Route gefunden:")
                for i, value in enumerate(nr):
                    print(f"Koordinaten des {i + 1}. Außenposten: x:{value[0]}, y:{value[1]}")
            elif normal_r.lower() == "stopp":
                break
            else:
                print("Dann tut es mir leid, dass ich nicht helfen konnte.")



dist_matrix = []
original = []
amnt_pts = 0
start()
