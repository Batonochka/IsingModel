import matplotlib.pyplot as plt
import numpy as np
import pandas as pd

def C(J, T):
    return (J/T)**2 / np.cosh(J/T)**2

def E(J, T):
    return - J * np.tanh(J/T)

def M(J, T, B):
    a = np.exp(J/T) * np.sinh(B/T)
    b = np.sqrt(np.exp(2*J/T)*np.sinh(B/T)**2 + np.exp(-2*J/T))
    return a/b

def read_energies(filename):
    df = pd.read_csv(filename)
    plt.figure(figsize=(10,10))
    plt.plot(df["step"], df["energy"], color='blue')
    plt.xlabel("Номер итерации")
    plt.ylabel("Энергии")
    plt.title("Энергии в алгоритме Зверополиса")
    plt.grid(visible=True, which='both')
    plt.savefig("Energy.png")

def read_spins(filename):
    df = pd.read_csv(filename)
    spins_list = []
    for s in df["spins"]:
        arr  = np.array([int(x) for x in s.split(',')])
        spins_list.append(arr)
    spin_matrix = np.array(spins_list).T
    spin_matrix = (spin_matrix + 1) // 2
    plt.figure(figsize=(10,10))
    plt.imshow(spin_matrix, cmap='gray_r', aspect='auto', interpolation='none')
    plt.xlabel("Итерация")
    plt.ylabel("Индекс спина")
    plt.savefig("Spins.png")

def read_data(filename):
    df = pd.read_csv(filename)
    fig, ax = plt.subplots(3,1,figsize=(30,10),sharex='all')
    J = 1
    B = 0
    energy = E(J, df["temperature"])
    momentum = M(J, df["temperature"], B)
    heat_cap = C(J, df["temperature"])
    ax[0].plot(df["temperature"], df["energy"], color='blue', label='numeric')
    ax[0].plot(df["temperature"], energy, color='green', label='analytic')
    ax[0].legend()
    ax[1].plot(df["temperature"], df["magnetic_momentum"],color='blue',label='numeric')
    ax[1].plot(df["temperature"], momentum, color='green', label='analytic')
    ax[1].legend()
    ax[2].plot(df['temperature'], df['heat_capacity'],color='blue',label='numeric')
    ax[2].plot(df["temperature"], heat_cap, color='green', label='analytic')
    ax[2].legend()
    ax[2].set_xlabel("Температура")
    ax[0].set_ylabel("Энергия")
    ax[1].set_ylabel("Магнитный момент")
    ax[2].set_ylabel("Теплоемкость")
    ax[0].grid(visible=True, which='both')
    ax[1].grid(visible=True, which='both')
    ax[2].grid(visible=True, which='both')
    plt.show()
    plt.savefig("simulation.png")


if __name__ == "__main__":
    spin_file = "Spins.csv"
    energy_file = "Energies.csv"
    data_file = "systemdata2d.csv"
    # read_energies(energy_file)
    # read_spins(spin_file)
    read_data(data_file)