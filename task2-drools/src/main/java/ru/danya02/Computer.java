package ru.danya02;

public class Computer {
    private int Architecture;
    private String Role;

    public Computer(int architecture, String role) {
        Architecture = architecture;
        Role = role;
    }

    public int getArchitecture() {
        return Architecture;
    }

    public String getRole() {
        return Role;
    }
}