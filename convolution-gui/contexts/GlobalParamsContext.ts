import { GUIParams } from '@/bindings/GUIParams';
import { Dispatch, SetStateAction } from 'react';
import { createContext } from 'react';

export const GlobalParametersContext = createContext<{
	parameters: GUIParams;
	setParameters: Dispatch<SetStateAction<GUIParams>>;
} | null>(null);
