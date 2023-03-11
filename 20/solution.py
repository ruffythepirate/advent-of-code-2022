import fileinput

class Node:
    def __init__(self, number=None):
        self.number = number
        self.next = None
        self.prev = None

def link_all_nodes(all_nodes):
    current_index = 0
    list_length = len(all_nodes)
    while current_index < list_length - 1:
        current_node = all_nodes[current_index]
        next_node = all_nodes[current_index + 1]
        current_node.next = next_node
        next_node.prev = current_node
        current_index += 1
    all_nodes[0].prev = all_nodes[-1]
    all_nodes[-1].next = all_nodes[0]

def create_node_list(all_numbers):
    all_nodes = []
    for number in all_numbers:
        node = Node(number)
        all_nodes.append(node)
    link_all_nodes(all_nodes)
    return all_nodes

def read_input():
    all_numbers = []
    for line in fileinput.input():
        # convert to int
        number = int(line)
        all_numbers.append(number)
    return all_numbers

def swap_node_left(current_node):
    left_node = current_node.prev
    right_node = current_node.next
    left_node.prev.next = current_node
    current_node.prev = left_node.prev
    current_node.next = left_node
    left_node.prev = current_node
    left_node.next = right_node
    right_node.prev = left_node

def swap_node_right(current_node):
    left_node = current_node.prev
    right_node = current_node.next
    right_node.next.prev = current_node
    current_node.next = right_node.next
    current_node.prev = right_node
    right_node.next = current_node
    right_node.prev = left_node
    left_node.next = right_node

def move_node(current_node, num_moves):
    while num_moves != 0:
        if num_moves < 0:
            swap_node_left(current_node)
            num_moves += 1
        elif num_moves > 0:
            swap_node_right(current_node)
            num_moves -= 1

def move_all_nodes(all_nodes):
    for node in all_nodes:
        move_node(node, node.number)

def get_zero_node(all_nodes):
    for node in all_nodes:
        if node.number == 0:
            return node

def peek_node_at_distance(current_node, distance):
    node = current_node
    for _ in range(distance):
        node = node.next
    return node

all_input = read_input()
all_nodes = create_node_list(all_input)
move_all_nodes(all_nodes)
zero_node = get_zero_node(all_nodes)
nodes_to_print = [peek_node_at_distance(zero_node, 1000),
                  peek_node_at_distance(zero_node, 2000),
                  peek_node_at_distance(zero_node, 3000)]

print(' '.join([str(node.number) for node in nodes_to_print]))
print(sum(map(lambda node: node.number, nodes_to_print)))

import unittest

class TestSolution(unittest.TestCase):
    def test_solution(self):
        all_nodes = create_node_list([1, 2, 3, 4, 5])
        self.assertEqual(1, all_nodes[0].number)
        for node in all_nodes:
            self.assertIsNotNone(node.next)
            self.assertIsNotNone(node.prev)

if __name__ == '__main__':
    unittest.main()





