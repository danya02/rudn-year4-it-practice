
package ru.danya02;

import java.util.List;

import org.drools.ruleunits.api.RuleUnitProvider;
import org.drools.ruleunits.api.RuleUnitInstance;
import org.junit.Test;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import static java.util.stream.Collectors.toList;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

public class RuleTest {

    static final Logger LOG = LoggerFactory.getLogger(RuleTest.class);

    @Test
    public void test() {
        LOG.info("Creating RuleUnit");
        ComputerUnit computerUnit = new ComputerUnit();

        RuleUnitInstance<ComputerUnit> instance = RuleUnitProvider.get().createRuleUnitInstance(computerUnit);
        try {
            LOG.info("Insert data");
            computerUnit.getMeasurements().add(new Measurement("color", "red"));
            computerUnit.getMeasurements().add(new Measurement("color", "green"));
            computerUnit.getMeasurements().add(new Measurement("color", "blue"));

            LOG.info("Run query. Rules are also fired");
            List<Measurement> queryResult = instance.executeQuery("FindColor").toList("$m");

            assertEquals(3, queryResult.size());
            assertTrue("contains red", computerUnit.getControlSet().contains("red"));
            assertTrue("contains green", computerUnit.getControlSet().contains("green"));
            assertTrue("contains blue", computerUnit.getControlSet().contains("blue"));
        } finally {
            instance.close();
        }
    }
}