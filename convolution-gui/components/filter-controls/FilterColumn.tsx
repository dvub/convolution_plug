export default function FilterColumn({
	children,
}: Readonly<{
	children: React.ReactNode;
}>) {
	return (
		<div className='w-[33%] h-full flex flex-col items-center justify-between bg-zinc-500 rounded-sm'>
			{children}
		</div>
	);
}
