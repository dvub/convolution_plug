export default function FilterColumn({
	children,
}: Readonly<{
	children: React.ReactNode;
}>) {
	return (
		<div className='w-[33%] flex flex-col items-center rounded-sm h-full'>
			{children}
		</div>
	);
}
