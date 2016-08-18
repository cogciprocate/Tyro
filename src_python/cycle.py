# from enum import Enum
from _libtyro import ffi, lib as libtyro
import gym

# Declare type "constants" (for ffi):
TYPEID_FLOAT32 = 0 # float
TYPEID_FLOAT64 = 1 # double
TYPEID_INT32   = 2 # int
TYPEID_INT64   = 3 # long

print('Creating new Tyro...')
tyro = ffi.new('Tyro', libtyro.new_tyro())

print('Warming up Tyro...')
assert libtyro.add_100(tyro[0], 999199) == 999299

# Reward cumulitive sum:
ttl_reward = 0.0

# env = gym.make('CartPole-v0')
# env = gym.make('LunarLander-v2')
# env = gym.make('FrozenLake-v0')
env = gym.make('Pendulum-v0')

# Set delta-t:
env.dt = .006

print("Action space: {}".format(env.action_space))
print("Observation space: {}".format(env.observation_space))

print('Running Simulation...\n')

for i_episode in range(1):
    observation = env.reset()
    episode_reward = 0.0

    for t in range(30):
        env.render()
        print("Observation as {}: {}".format(type(observation), observation))

        # obs_list = observation.tolist()
        # print("Observation as list: {}".format(obs_list))

        # obs_tuple = tuple(obs_list)
        # print("Observation as tuple: {}".format(obs_list))

        # obs_ctype = ffi.new("double[4]", obs_tuple)
        # libtyro.print_array(obs_ctype)


        libtyro.print_array(ffi.cast("double*", observation.ctypes.data), 
            observation.size, TYPEID_FLOAT64)

        action = env.action_space.sample()
        print("Taking action: {}".format(action))
        observation, reward, done, info = env.step(action)

        ttl_reward += reward
        libtyro.add_reward(tyro[0], reward)
        episode_reward += reward

        if done:
            print("Episode {} finished after {} timesteps with reward {}\n".format(i_episode, t + 1, episode_reward))
            break


print('Total reward is: {} ({})'.format(libtyro.get_reward(tyro[0]), ttl_reward))
assert (libtyro.get_reward(tyro[0]) - ttl_reward) < 0.0001

# Return threads and drop:
libtyro.drop_tyro(tyro[0])