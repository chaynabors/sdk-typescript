package com.strands

import kotlin.test.Test
import kotlin.test.assertNotNull

class AgentTest {

    @Test
    fun constructsWithDefaults() {
        val agent = Agent()
        assertNotNull(agent)
        agent.close()
    }
}
