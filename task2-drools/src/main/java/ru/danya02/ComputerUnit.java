
package ru.danya02;

import java.util.HashSet;
import java.util.Set;

import org.drools.ruleunits.api.DataSource;
import org.drools.ruleunits.api.DataStore;
import org.drools.ruleunits.api.RuleUnitData;

public class ComputerUnit implements RuleUnitData {

    private final DataStore<Computer> computers;
    private final Set<String> controlSet = new HashSet<>();

    public ComputerUnit() {
        this(DataSource.createStore());
    }

    public ComputerUnit(DataStore<Computer> computers) {
        this.computers = computers;
    }

    public DataStore<Computer> getComputers() {
        return computers;
    }

    public Set<String> getControlSet() {
        return controlSet;
    }
}
