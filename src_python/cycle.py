# from enum import Enum
import numpy as np
from _libtyro import ffi, lib as libtyro
import gym
from tyro import Tyro, TYPEID_FLOAT32, TYPEID_FLOAT64, TYPEID_INT32, TYPEID_INT64


print('Creating new Tyro...')
with Tyro() as tyro:
    # print('Warming up Tyro...')
    # assert tyro.add_100(999199) == 999299

    # Reward cumulitive sum:
    ttl_reward = 0.0

    # env = gym.make('CartPole-v0')
    # env = gym.make('LunarLander-v2')
    # env = gym.make('FrozenLake-v0')
    env = gym.make('Pendulum-v0')

    # Set delta-t:
    env.dt = .006

    tyro.set_encoder_ranges(env.observation_space.low, env.observation_space.high)

    print("Action space: {}".format(env.action_space))
    print("Observation space: {} ({})(\n\t{}, \n\t{}\n)".format(env.observation_space, env.observation_space.low.dtype,
        env.observation_space.low, env.observation_space.high))
    # print("Type of len(env.observation_space.low): {}".format(type(len(env.observation_space.low)))) // 'int'

    print('Running Simulation...\n')

    for i_episode in range(100):
        observation = env.reset()
        episode_reward = 0.0

        for t in range(300000):
            env.render()
            # print("Observation as {}: {}".format(type(observation), observation))

            # obs_list = observation.tolist()
            # print("Observation as list: {}".format(obs_list))

            # obs_tuple = tuple(obs_list)
            # print("Observation as tuple: {}".format(obs_list))

            # obs_ctype = ffi.new("double[4]", obs_tuple)
            # tyro.print_array(obs_ctype)

            # tyro.print_observation(observation)
            tyro.push_observation(observation)
            tyro.cycle()

            action = env.action_space.sample()
            # print("Taking action: {}".format(action))
            observation, reward, done, info = env.step(action)

            ttl_reward += reward
            tyro.add_reward(reward)
            episode_reward += reward

            # TODO: Check tyro exit status (if it's exiting).

            if done:
                print("Episode {} finished after {} timesteps with reward {}\n"
                    .format(i_episode, t + 1, episode_reward))
                break


    print('Total reward is: {} ({})'.format(tyro.get_reward(), ttl_reward))
    assert (tyro.get_reward() - ttl_reward) < 0.001
    