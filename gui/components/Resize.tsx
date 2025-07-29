import { sendToPlugin } from '@/lib';
import { useState, useEffect, useRef } from 'react';

const DEFAULT_SIZE = { width: 600, height: 600 };
const CURSOR_CLASSES = {
	corner: 'cursor-se-resize',
};

// TODO: add resizing along bottom and side of window

export function Resize() {
	const resizeData = useRef({
		startPos: { x: 0, y: 0 },
		startSize: { width: 0, height: 0 },
	});

	const [isResizing, setIsResizing] = useState(false);

	function handleMouseDown(e: React.PointerEvent) {
		setIsResizing(true);
		resizeData.current = {
			startPos: { x: e.clientX, y: e.clientY },
			startSize: {
				width: window.innerWidth,
				height: window.innerHeight,
			},
		};
		document.body.classList.add(CURSOR_CLASSES.corner);
	}

	function handleReset() {
		sendToPlugin({
			type: 'resize',
			data: {
				width: DEFAULT_SIZE.width,
				height: DEFAULT_SIZE.height,
			},
		});
	}

	useEffect(() => {
		function handleMouseUp() {
			document.body.classList.remove(...Object.values(CURSOR_CLASSES));
			setIsResizing(false);
		}
		function handleMouseMove(e: MouseEvent) {
			if (!isResizing) {
				return;
			}

			const deltaX = e.clientX - resizeData.current.startPos.x;
			const deltaY = e.clientY - resizeData.current.startPos.y;

			const width = Math.max(
				DEFAULT_SIZE.width,
				resizeData.current.startSize.width + deltaX
			);
			const height = Math.max(
				DEFAULT_SIZE.height,
				resizeData.current.startSize.height + deltaY
			);

			sendToPlugin({
				type: 'resize',
				data: {
					width,
					height,
				},
			});
		}

		if (isResizing) {
			// Add global listeners when resizing starts
			window.addEventListener('pointermove', handleMouseMove);
			window.addEventListener('pointerup', handleMouseUp);
		}

		// Cleanup function: remove listeners when component unmounts or isResizing becomes false
		return () => {
			window.removeEventListener('pointermove', handleMouseMove);
			window.removeEventListener('pointerup', handleMouseUp);
		};
	}, [isResizing]);

	return (
		<svg
			viewBox='0 0 10 10'
			width='2.5rem'
			height='2.5rem'
			onPointerDown={handleMouseDown}
			onDoubleClick={handleReset}
			className='absolute bottom-0 right-0 cursor-se-resize'
		>
			<path d='M 10 0 L 10 10 L 0 10 Z' fill='#0d100f' />
		</svg>
	);
}
