<script lang="ts">
	import {
		Button,
		Card,
		CardBody,
		CardFooter,
		CardHeader,
		CardSubtitle,
		CardText,
		CardTitle,
		Container,
		Input,
	} from 'sveltestrap';

	import { invoke } from '@tauri-apps/api/tauri';

	let message_in: string = '';
	let length_in: string = '8';
	let result: string = '';
	const handleText = async () => {
		result = await invoke('my_custom_command', {
			invokeMessage: message_in, // pass message and get response to it
		})
	};
	const handlePassword = async () => {
		result = await invoke('generate_password', {
			invokePassword: length_in,	// pass length and get password back
		})
	};
</script>

<main>
	<Container>
		<Card class="mb-3">
			<CardHeader>
				<CardTitle>Tauri Experimenting</CardTitle>
			</CardHeader>
			<CardBody>
				<fieldset>
					<label for='msg' alt="To send a message: Enter the message and press the 'Send Message' button.">Message Text</label>
					<Input type="text" id='msg' bind:value={message_in} />
				</fieldset>
				<fieldset>
					<label for='len' alt="To generate a password: Enter the length and press the 'Get Password' button.">Password Length</label>
					<Input type="text" id='len' bind:value={length_in} />
				</fieldset>

				<fieldset>
					<Button color="primary" on:click={handleText}>Send Message</Button>
					<Button color="primary" on:click={handlePassword}>Get Password</Button>
				</fieldset>
			</CardBody>
			<CardFooter>
				{#if result.length !== 0}
					{result}
				{:else}
					No result yet.
				{/if}
			</CardFooter>
		</Card>
	</Container>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
