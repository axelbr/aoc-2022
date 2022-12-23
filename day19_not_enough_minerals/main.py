import re

import cvxpy as cp
import numpy as np


def parse_blueprints(blueprints: str):
    def parse_blueprint(data: str):
        lines = data.split('. ')
        blueprint = dict()
        for line in map(lambda l: l.strip(), lines):
            robot = line.split(" ")[1]
            blueprint[robot] = dict()
            for res in ['ore', 'obsidian', 'geode', 'clay']:
                costs = re.findall(f'\d+ {res}', line[line.index('costs '):])
                if len(costs) == 1:
                    blueprint[robot][res] = int(costs[0].split(' ')[0])
        return blueprint

    with open(blueprints, 'r') as f:
        data = f.read()
        return [parse_blueprint(b[b.index(': ') + 1:]) for b in data.splitlines()]


def solve(blueprint: dict, minutes: int) -> dict:
    robot_orders = cp.Variable((minutes + 1, len(blueprint)), integer=True)
    cost_matrix = np.zeros((len(blueprint), len(blueprint)))
    resources = ['ore', 'clay', 'obsidian', 'geode']
    for robot in blueprint:
        for res in blueprint[robot]:
            i, j = resources.index(res), resources.index(robot)
            cost_matrix[i, j] = blueprint[robot][res]

    resources = [np.zeros(4)]
    robots = [np.array([1, 0, 0, 0])]
    constraints = []
    for t in range(1, minutes + 1):
        new_res = resources[t - 1] + robots[t - 1]
        robots.append(robots[t - 1] + robot_orders[t])
        costs = cost_matrix @ robot_orders[t]
        resources.append(new_res - costs)

        constraints.append(robot_orders[t] >= 0)
        constraints.append(cp.sum(robot_orders[t]) <= 1)
        constraints.append(costs <= resources[t - 1])

    objective = resources[-1][-1]
    problem = cp.Problem(cp.Maximize(objective), constraints)
    problem.solve()
    return dict(
        robot_orders=robot_orders.value[1:],
        objective=objective.value,
        resources=[r.value for r in resources[1:]],
        robots=[r.value for r in robots[1:]]
    )


if __name__ == '__main__':
    blueprints = parse_blueprints('./input/task_1.txt')
    total_quality_level = 0
    for i, bp in enumerate(blueprints):
        solution = solve(bp, 24)
        print(f'Blueprint {i + 1}: {solution["objective"]} geodes')
        total_quality_level += solution['objective'] * (i + 1)
    print(f'[Task 1] Total quality level: {total_quality_level}')

    # Task 2
    left_blueprints = blueprints[:3]
    total_geodes = 1
    for i, bp in enumerate(left_blueprints):
        solution = solve(bp, minutes=32)
        print(f'Blueprint {i + 1}: {solution["objective"]} geodes')
        total_geodes *= solution['objective']
    print(f'[Task 2] Total geodes of first three blueprints: {total_geodes}')
