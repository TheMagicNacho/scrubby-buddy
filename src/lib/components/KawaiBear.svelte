<script lang="ts">
	type BearState = 'waiting' | 'talking' | 'looking';

	interface Props {
		state?: BearState;
		message?: string;
	}

	let { state = 'waiting', message = '' }: Props = $props();
</script>

<div class="bear-container">
	{#if message && state === 'talking'}
		<div class="speech-bubble">
			<p>{message}</p>
			<div class="bubble-tail"></div>
		</div>
	{/if}
	<div
		class="bear"
		class:looking={state === 'looking'}
		class:talking={state === 'talking'}
		class:waiting={state === 'waiting'}
	>
		<!-- Ears -->
		<div class="ear left"></div>
		<div class="ear right"></div>

		<!-- Head/Face -->
		<div class="head">
			<!--            &lt;!&ndash; Inner ears &ndash;&gt;-->
			<!--            <div class="inner-ear left"></div>-->
			<!--            <div class="inner-ear right"></div>-->

			<!-- Eyes -->
			<div
				class="eyes"
				class:eyes-looking={state === 'looking'}
				class:eyes-talking={state === 'talking'}
				class:eyes-waiting={state === 'waiting'}
			>
				<div class="eye left">
					<div class="pupil"></div>
					<div class="sparkle"></div>
				</div>
				<div class="eye right">
					<div class="pupil"></div>
					<div class="sparkle"></div>
				</div>
			</div>

			<!-- Blush -->
			<div class="blush left"></div>
			<div class="blush right"></div>

			<!-- Nose -->
			<div class="nose"></div>

			<!-- Mouth -->
			<div class="mouth" class:mouth-talking={state === 'talking'}></div>
		</div>

		<!-- Body -->
		<div class="body">
			<div class="belly"></div>
			<!-- Arms -->
			<div class="arm left"></div>
			<div class="arm right"></div>
			<!-- Feet -->
			<div class="foot left"></div>
			<div class="foot right"></div>
		</div>
	</div>
</div>

<style>
	.bear-container {
		position: fixed;
		bottom: 20px;
		left: 20px;
		z-index: 1000;
	}

	/* Speech bubble — absolutely positioned above the bear so it never shifts the bear */
	.speech-bubble {
		background: linear-gradient(135deg, #ffffff 0%, #ffd6e0 100%);
		border-radius: 20px;
		border: 3px solid #ffb6c1;
		padding: 15px 20px;
		max-width: 200px;
		box-shadow: 0 8px 25px rgba(255, 20, 147, 0.15);
		position: absolute;
		bottom: 100%;
		left: 0;
		margin-bottom: 18px;
	}

	.speech-bubble::before {
		content: '💭';
		position: absolute;
		top: -10px;
		right: -10px;
		font-size: 1.2em;
		animation: sparkle 2s ease-in-out infinite;
	}

	.speech-bubble p {
		margin: 0;
		font-size: 13px;
		font-weight: 600;
		color: #4a0e4e;
		line-height: 1.4;
		font-family: 'Nunito', sans-serif;
	}

	.bubble-tail {
		position: absolute;
		bottom: -15px;
		left: 20px;
		width: 0;
		height: 0;
		border-left: 15px solid transparent;
		border-right: 15px solid transparent;
		border-top: 15px solid #ffb6c1;
	}

	.bubble-tail::after {
		content: '';
		position: absolute;
		bottom: 3px;
		left: -12px;
		width: 0;
		height: 0;
		border-left: 12px solid transparent;
		border-right: 12px solid transparent;
		border-top: 12px solid #ffffff;
	}

	/* Dark mode support */
	@media (prefers-color-scheme: dark) {
		.speech-bubble {
			background: #3a3a3a;
			box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
		}

		.speech-bubble p {
			color: #f6f6f6;
		}

		.bubble-tail {
			border-top-color: #3a3a3a;
		}
	}

	.bear {
		position: relative;
		width: 80px;
		height: 100px;
		cursor: pointer;
		animation: float 3s ease-in-out infinite;
	}

	.bear.waiting {
		animation: float 3s ease-in-out infinite;
	}

	.bear.talking {
		animation: bounce 0.5s ease-in-out infinite;
	}

	.bear.looking {
		animation: tilt 2s ease-in-out infinite;
	}

	@keyframes float {
		0%,
		100% {
			transform: translateY(0);
		}
		50% {
			transform: translateY(-5px);
		}
	}

	@keyframes bounce {
		0%,
		100% {
			transform: translateY(0) scale(1);
		}
		50% {
			transform: translateY(-3px) scale(1.02);
		}
	}

	@keyframes tilt {
		0%,
		100% {
			transform: rotate(0deg);
		}
		25% {
			transform: rotate(-5deg);
		}
		75% {
			transform: rotate(5deg);
		}
	}

	/* Ears */
	.ear {
		position: absolute;
		width: 24px;
		height: 24px;
		background: #8b6914;
		border-radius: 50%;
		top: 0;
		z-index: 1;
	}

	.ear.left {
		left: 8px;
	}

	.ear.right {
		right: 8px;
	}

	/*.inner-ear {*/
	/*    position: absolute;*/
	/*    width: 12px;*/
	/*    height: 12px;*/
	/*    background: #FFCBA4;*/
	/*    border-radius: 50%;*/
	/*    top: -7px;*/
	/*    z-index: 1;*/
	/*}*/

	/*.inner-ear.left {*/
	/*    left: 10px;*/
	/*}*/

	/*.inner-ear.right {*/
	/*    right: 10px;*/
	/*}*/

	/* Head */
	.head {
		position: absolute;
		width: 70px;
		height: 60px;
		background: linear-gradient(145deg, #c4a24c, #8b6914);
		border-radius: 50% 50% 45% 45%;
		top: 10px;
		left: 5px;
		z-index: 2;
		box-shadow: inset -3px -3px 10px rgba(0, 0, 0, 0.1);
	}

	/* Eyes */
	.eyes {
		position: absolute;
		display: flex;
		justify-content: space-between;
		width: 40px;
		top: 20px;
		left: 15px;
	}

	.eye {
		width: 14px;
		height: 14px;
		background: #535bf2;
		border-radius: 50%;
		position: relative;
		overflow: hidden;
	}

	.pupil {
		position: absolute;
		width: 8px;
		height: 8px;
		background: #0f0f0f;
		border-radius: 50%;
		top: 3px;
		left: 3px;
		transition: all 0.3s ease;
	}

	.sparkle {
		position: absolute;
		width: 4px;
		height: 4px;
		background: #fff;
		border-radius: 50%;
		top: 3px;
		left: 7px;
		animation: sparkle 2s ease-in-out infinite;
	}

	@keyframes sparkle {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}

	/* Looking state - pupils move */
	.eyes-looking .pupil {
		animation: lookAround 2s ease-in-out infinite;
	}

	@keyframes lookAround {
		0%,
		100% {
			transform: translate(0, 0);
		}
		25% {
			transform: translate(3px, 0);
		}
		50% {
			transform: translate(3px, 2px);
		}
		75% {
			transform: translate(-2px, 0);
		}
	}

	/* Waiting state - occasional blink */
	.eyes-waiting .eye {
		animation: blink 4s ease-in-out infinite;
	}

	@keyframes blink {
		0%,
		45%,
		55%,
		100% {
			transform: scaleY(1);
		}
		50% {
			transform: scaleY(0.1);
		}
	}

	/* Talking state - happy eyes */
	.eyes-talking .eye {
		height: 6px;
		border-radius: 0 0 10px 10px;
		animation: happyEyes 0.3s ease-in-out infinite alternate;
	}

	@keyframes happyEyes {
		0% {
			transform: translateY(0);
		}
		100% {
			transform: translateY(1px);
		}
	}

	/* Blush */
	.blush {
		position: absolute;
		width: 12px;
		height: 8px;
		background: rgba(255, 150, 150, 0.6);
		border-radius: 50%;
		top: 32px;
		z-index: 3;
	}

	.blush.left {
		left: 5px;
	}

	.blush.right {
		right: 5px;
	}

	/* Nose */
	.nose {
		position: absolute;
		width: 10px;
		height: 8px;
		background: #5d4037;
		border-radius: 50%;
		top: 35px;
		left: 30px;
		z-index: 3;
	}

	/* Mouth */
	.mouth {
		position: absolute;
		width: 12px;
		height: 6px;
		border: 2px solid #5d4037;
		border-top: none;
		border-radius: 0 0 10px 10px;
		top: 43px;
		left: 29px;
		z-index: 3;
		background: transparent;
	}

	.mouth-talking {
		animation: talk 0.2s ease-in-out infinite;
	}

	@keyframes talk {
		0%,
		100% {
			height: 6px;
			width: 12px;
			left: 29px;
		}
		50% {
			height: 10px;
			width: 14px;
			left: 28px;
		}
	}

	/* Body */
	.body {
		position: absolute;
		width: 50px;
		height: 35px;
		background: linear-gradient(145deg, #c4a24c, #8b6914);
		border-radius: 50% 50% 45% 45%;
		top: 62px;
		left: 15px;
		z-index: 1;
	}

	.belly {
		position: absolute;
		width: 30px;
		height: 20px;
		background: #ffcba4;
		border-radius: 50%;
		top: 8px;
		left: 10px;
	}

	/* Arms */
	.arm {
		position: absolute;
		width: 14px;
		height: 20px;
		background: #8b6914;
		border-radius: 40%;
		top: 5px;
		z-index: 0;
	}

	.arm.left {
		left: -8px;
		transform: rotate(20deg);
		animation: waveLeft 2s ease-in-out infinite;
	}

	.arm.right {
		right: -8px;
		transform: rotate(-20deg);
		animation: waveRight 2s ease-in-out infinite;
	}

	@keyframes waveLeft {
		0%,
		100% {
			transform: rotate(20deg);
		}
		50% {
			transform: rotate(10deg);
		}
	}

	@keyframes waveRight {
		0%,
		100% {
			transform: rotate(-20deg);
		}
		50% {
			transform: rotate(-10deg);
		}
	}

	.bear.talking .arm.right {
		animation: wave 0.4s ease-in-out infinite;
	}

	@keyframes wave {
		0%,
		100% {
			transform: rotate(-20deg);
		}
		50% {
			transform: rotate(-45deg);
		}
	}

	/* Feet */
	.foot {
		position: absolute;
		width: 16px;
		height: 10px;
		background: #8b6914;
		border-radius: 50%;
		bottom: -5px;
	}

	.foot.left {
		left: 5px;
	}

	.foot.right {
		right: 5px;
	}
</style>
