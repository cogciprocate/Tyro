from _libtyro import ffi, lib as libtyro
import gym

print('Creating new Tyro...')
tyro = ffi.new('Tyro', libtyro.new_tyro())

print('Warming up Tyro...')
libtyro.hello(libtyro.add_100(tyro[0], 999199))


print('Running Simulation...\n')

ttl_reward = 0.0
env = gym.make('CartPole-v0')

for i_episode in range(5):
    observation = env.reset()

    for t in range(100):
        env.render()
        print(observation)

        action = env.action_space.sample()
        print("Taking action: {}".format(action))
        observation, reward, done, info = env.step(action)

        ttl_reward += reward
        libtyro.add_reward(tyro[0], reward)

        if done:
            print("Episode {} finished after {} timesteps\n".format(i_episode, t + 1))
            break


assert libtyro.get_reward(tyro[0]) == ttl_reward

print('Total reward is: {}'.format(libtyro.get_reward(tyro[0])))

libtyro.drop_tyro(tyro[0])