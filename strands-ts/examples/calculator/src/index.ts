import { Agent, BedrockModel, tool } from '@strands-agents/sdk'
import { z } from 'zod'

const calculator = tool({
  name: 'calculator',
  description: 'Evaluate a math expression and return the numeric result.',
  inputSchema: z.object({
    expression: z.string().describe('A math expression to evaluate'),
  }),
  callback: (input) => {
    const result = Function(`"use strict"; return (${input.expression})`)()
    return `${result}`
  },
})

async function main() {
  const model = new BedrockModel()
  const agent = new Agent({
    systemPrompt: 'You are a helpful assistant with a calculator tool.',
    model,
    tools: [calculator],
  })

  const result = await agent.invoke('What is the square root of 1764?')
  console.log(`\nStop reason: ${result.stopReason}`)
}

await main().catch(console.error)
